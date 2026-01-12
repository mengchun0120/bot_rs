use crate::config::{game_config::*, game_map_config::*, game_obj_config::*};
use crate::game::game_obj::*;
use crate::game_utils::{despawn_pool::*, game_lib::*, game_obj_lib::*};
use crate::misc::{collide::*, my_error::*, utils::*};
use bevy::prelude::*;

use std::collections::HashSet;
use std::path::Path;

#[derive(Resource)]
pub struct GameMap {
    pub cell_size: f32,
    pub width: f32,
    pub height: f32,
    pub map: Vec<Vec<HashSet<Entity>>>,
    pub max_collide_span: f32,
    min_origin: Vec2,
    max_origin: Vec2,
    origin: Vec2,
    visible_span: Vec2,
    visible_region: MapRegion,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct MapPos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, Resource, Eq, PartialEq, Default)]
pub struct MapRegion {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl GameMap {
    pub fn new(cell_size: f32, row_count: usize, col_count: usize) -> Self {
        Self {
            cell_size,
            width: col_count as f32 * cell_size,
            height: row_count as f32 * cell_size,
            map: vec![vec![HashSet::new(); col_count]; row_count],
            max_collide_span: 0.0,
            min_origin: Vec2::default(),
            max_origin: Vec2::default(),
            origin: Vec2::default(),
            visible_span: Vec2::default(),
            visible_region: MapRegion::default(),
        }
    }

    pub fn load<P: AsRef<Path>>(
        map_path: P,
        cell_size: f32,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        commands: &mut Commands,
    ) -> Result<GameMap, MyError> {
        let map_config: GameMapConfig = read_json(map_path)?;
        let mut map = Self::new(cell_size, map_config.row_count, map_config.col_count);

        map.setup_origin(&game_lib.game_config, &map_config);
        map.setup_visible_region(&game_lib.game_config);

        map.add_obj_by_config(&map_config.player, game_lib, game_obj_lib, commands)?;

        for map_obj_config in map_config.objs.iter() {
            map.add_obj_by_config(map_obj_config, game_lib, game_obj_lib, commands)?;
        }

        Ok(map)
    }

    pub fn add_obj_by_config(
        &mut self,
        map_obj_config: &GameMapObjConfig,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let config_index = game_lib.get_game_obj_config_index(&map_obj_config.config_name)?;
        let pos = arr_to_vec2(&map_obj_config.pos);
        let direction = arr_to_vec2(&map_obj_config.direction).normalize();

        self.add_obj_by_index(
            config_index,
            &pos,
            &direction,
            game_lib,
            game_obj_lib,
            commands,
        )?;

        Ok(())
    }

    pub fn add_obj_by_index(
        &mut self,
        config_index: usize,
        pos: &Vec2,
        direction: &Vec2,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let obj_config = game_lib.get_game_obj_config(config_index);

        if !self.contains(pos, obj_config.collide_span) {
            let err_msg = format!("Position {:?} is outside of map", pos);
            error!(err_msg);
            return Err(MyError::Other(err_msg));
        }

        let (obj, entity) = GameObj::new(config_index, pos, direction, self, game_lib, commands)?;

        self.map[obj.map_pos.row][obj.map_pos.col].insert(entity);
        if self.max_collide_span < obj_config.collide_span {
            self.max_collide_span = obj_config.collide_span;
        }

        game_obj_lib.insert(entity, obj);

        Ok(())
    }

    #[inline]
    pub fn contains(&self, pos: &Vec2, collide_span: f32) -> bool {
        pos.x >= collide_span
            && pos.x + collide_span < self.width
            && pos.y >= collide_span
            && pos.y + collide_span < self.height
    }

    #[inline]
    pub fn get_map_pos(&self, pos: &Vec2) -> MapPos {
        MapPos {
            row: (pos.y / self.cell_size).floor() as usize,
            col: (pos.x / self.cell_size).floor() as usize,
        }
    }

    #[inline]
    pub fn get_screen_pos(&self, pos: &Vec2) -> Vec2 {
        pos - self.origin
    }

    #[inline]
    pub fn viewport_to_world(&self, pos: &Vec2) -> Vec2 {
        pos + self.origin
    }

    #[inline]
    pub fn check_pos_visible(&self, pos: &Vec2) -> bool {
        (pos.x - self.origin.x).abs() <= self.visible_span.x
            && (pos.y - self.origin.y).abs() <= self.visible_span.y
    }

    pub fn get_bot_new_pos(
        &self,
        entity: &Entity,
        obj: &GameObj,
        game_obj_lib: &GameObjLib,
        game_lib: &GameLib,
        time: &Time,
    ) -> (bool, Vec2) {
        let obj_config = game_lib.get_game_obj_config(obj.config_index);
        let new_pos = obj.pos + obj.direction * obj_config.speed * time.delta_secs();

        let (collide_bounds, new_pos) = get_bot_pos_after_collide_bounds(
            &new_pos,
            obj_config.collide_span,
            &obj.direction,
            self.width,
            self.height,
        );

        let (collide_obj, new_pos) = self.get_bot_pos_after_collide_objs(
            entity,
            obj,
            &new_pos,
            obj_config,
            game_obj_lib,
            game_lib,
        );

        (collide_bounds || collide_obj, new_pos)
    }

    pub fn update_origin(
        &mut self,
        origin: &Vec2,
        game_obj_lib: &GameObjLib,
        game_lib: &GameLib,
        despawn_pool: &mut DespawnPool,
        commands: &mut Commands,
    ) {
        self.origin.x = origin.x.clamp(self.min_origin.x, self.max_origin.x);
        self.origin.y = origin.y.clamp(self.min_origin.y, self.max_origin.y);

        let new_visible_region = self.get_visible_region(&self.origin);

        self.hide_offscreen(
            &new_visible_region,
            game_obj_lib,
            game_lib,
            despawn_pool,
            commands,
        );
        self.update_onscreen(&new_visible_region, game_obj_lib, commands);
        self.show_newscreen(&new_visible_region, game_obj_lib, commands);

        self.visible_region = new_visible_region;
    }

    pub fn relocate(&mut self, entity: Entity, old_pos: &MapPos, new_pos: &MapPos) {
        self.map[old_pos.row][old_pos.col].remove(&entity);
        self.map[new_pos.row][new_pos.col].insert(entity);
    }

    #[inline]
    pub fn row_count(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn col_count(&self) -> usize {
        self.map[0].len()
    }

    #[inline]
    pub fn remove(&mut self, entity: &Entity, map_pos: &MapPos) {
        if !self.map[map_pos.row][map_pos.col].remove(entity) {
            error!(
                "Cannot remove entity {:?} from GameMap at position {:?}",
                entity, map_pos
            );
        }
    }

    fn setup_origin(&mut self, game_config: &GameConfig, map_config: &GameMapConfig) {
        let player_pos = arr_to_vec2(&map_config.player.pos);
        self.min_origin = Vec2::new(
            game_config.window_width() / 2.0,
            game_config.window_height() / 2.0,
        );
        self.max_origin = Vec2::new(
            self.width - self.min_origin.x,
            self.height - self.min_origin.y,
        );
        self.origin.x = player_pos.x.clamp(self.min_origin.x, self.max_origin.x);
        self.origin.y = player_pos.y.clamp(self.min_origin.y, self.max_origin.y);
    }

    fn setup_visible_region(&mut self, game_config: &GameConfig) {
        self.visible_span.x = game_config.window_width() / 2.0 + game_config.window_ext_size;
        self.visible_span.y = game_config.window_height() / 2.0 + game_config.window_ext_size;
        self.visible_region = self.get_visible_region(&self.origin);
    }

    #[inline]
    fn get_visible_region(&self, origin: &Vec2) -> MapRegion {
        self.get_region(
            origin.x - self.visible_span.x,
            origin.y - self.visible_span.y,
            origin.x + self.visible_span.x,
            origin.y + self.visible_span.y,
        )
    }

    #[inline]
    fn get_row(&self, y: f32) -> usize {
        let i = (y / self.cell_size).floor() as i32;
        i.clamp(0, (self.row_count() - 1) as i32) as usize
    }

    #[inline]
    fn get_col(&self, x: f32) -> usize {
        let i = (x / self.cell_size).floor() as i32;
        i.clamp(0, (self.col_count() - 1) as i32) as usize
    }

    fn get_bot_pos_after_collide_objs(
        &self,
        entity: &Entity,
        obj: &GameObj,
        new_pos: &Vec2,
        obj_config: &GameObjConfig,
        game_obj_lib: &GameObjLib,
        game_lib: &GameLib,
    ) -> (bool, Vec2) {
        let mut collide = false;
        let collide_region =
            self.get_collide_region_bot(&obj.pos, new_pos, obj_config.collide_span);
        let mut pos = new_pos.clone();
        let func = |e: &Entity| -> bool {
            if entity == e {
                return true;
            }

            let Some(obj2) = game_obj_lib.get(e) else {
                error!("Cannot find entity in GameObjLib");
                return true;
            };
            let obj_config2 = game_lib.get_game_obj_config(obj2.config_index);

            if (obj_config2.obj_type != GameObjType::Bot
                && obj_config2.obj_type != GameObjType::Tile)
                || obj_config2.collide_span == 0.0
            {
                return true;
            }

            let (collide_obj, corrected_pos) = get_bot_pos_after_collide_obj(
                &pos,
                obj_config.collide_span,
                &obj.direction,
                &obj2.pos,
                obj_config2.collide_span,
            );

            if collide_obj {
                collide = true;
            }

            pos = corrected_pos;

            true
        };

        self.run_on_region(&collide_region, func);

        (collide, pos)
    }

    fn hide_offscreen(
        &self,
        new_visible_region: &MapRegion,
        game_obj_lib: &GameObjLib,
        game_lib: &GameLib,
        despawn_pool: &mut DespawnPool,
        commands: &mut Commands,
    ) {
        let offscreen_regions = self.visible_region.sub(&new_visible_region);
        let func = |entity: &Entity| -> bool {
            let Some(config_index) = game_obj_lib.get(entity).map(|obj| obj.config_index) else {
                error!("Cannot find entity {:?} in GameObjLib", entity);
                return true;
            };
            let obj_type = game_lib.get_game_obj_config(config_index).obj_type;

            if obj_type == GameObjType::Missile || obj_type == GameObjType::Effect {
                despawn_pool.insert(entity.clone());
                return true;
            }

            commands
                .entity(entity.clone())
                .entry::<Visibility>()
                .and_modify(|mut v| *v = Visibility::Hidden);

            true
        };

        self.run_on_regions(&offscreen_regions, func);
    }

    fn update_onscreen(
        &self,
        new_visible_region: &MapRegion,
        game_obj_lib: &GameObjLib,
        commands: &mut Commands,
    ) {
        let onscreen_regions = self.visible_region.intersect(&new_visible_region);
        let func = |entity: &Entity| -> bool {
            let Some(obj) = game_obj_lib.get(entity) else {
                return true;
            };
            let screen_pos = self.get_screen_pos(&obj.pos);
            commands
                .entity(entity.clone())
                .entry::<Transform>()
                .and_modify(move |mut t| {
                    t.translation.x = screen_pos.x;
                    t.translation.y = screen_pos.y;
                });

            true
        };

        self.run_on_regions(&onscreen_regions, func);
    }

    fn show_newscreen(
        &self,
        new_visible_region: &MapRegion,
        game_obj_lib: &GameObjLib,
        commands: &mut Commands,
    ) {
        let newscreen_regions = new_visible_region.sub(&self.visible_region);
        let func = |entity: &Entity| -> bool {
            let Some(obj) = game_obj_lib.get(entity) else {
                return true;
            };
            let screen_pos = self.get_screen_pos(&obj.pos);
            let mut entity = commands.entity(entity.clone());

            entity.entry::<Transform>().and_modify(move |mut t| {
                t.translation.x = screen_pos.x;
                t.translation.y = screen_pos.y;
            });

            entity.entry::<Visibility>().and_modify(|mut v| {
                *v = Visibility::Visible;
            });

            true
        };

        self.run_on_regions(&newscreen_regions, func);
    }

    pub fn run_on_regions<F>(&self, regions: &Vec<MapRegion>, mut func: F) -> bool
    where
        F: FnMut(&Entity) -> bool,
    {
        for region in regions.iter() {
            if !self.run_on_region(region, &mut func) {
                return false;
            }
        }
        true
    }

    pub fn run_on_region<F>(&self, region: &MapRegion, mut func: F) -> bool
    where
        F: FnMut(&Entity) -> bool,
    {
        for row in region.start_row..=region.end_row {
            for col in region.start_col..=region.end_col {
                for entity in self.map[row][col].iter() {
                    if !func(entity) {
                        return false;
                    }
                }
            }
        }
        true
    }

    #[inline]
    fn get_collide_region_bot(
        &self,
        start_pos: &Vec2,
        end_pos: &Vec2,
        collide_span: f32,
    ) -> MapRegion {
        let span = self.max_collide_span + collide_span;
        self.get_region(
            start_pos.x.min(end_pos.x) - span,
            start_pos.y.min(end_pos.y) - span,
            start_pos.x.max(end_pos.x) + span,
            start_pos.y.max(end_pos.y) + span,
        )
    }

    #[inline]
    pub fn get_region(&self, left: f32, bottom: f32, right: f32, top: f32) -> MapRegion {
        MapRegion {
            start_row: self.get_row(bottom),
            end_row: self.get_row(top),
            start_col: self.get_col(left),
            end_col: self.get_col(right),
        }
    }
}

impl MapRegion {
    #[inline]
    pub fn contains(&self, pos: &MapPos) -> bool {
        pos.row >= self.start_row
            && pos.row <= self.end_row
            && pos.col >= self.start_col
            && pos.col <= self.end_col
    }

    pub fn sub(&self, other: &MapRegion) -> Vec<MapRegion> {
        let mut result = Vec::new();

        if self.start_row > other.end_row
            || self.end_row < other.start_row
            || self.start_col > other.end_col
            || self.end_col < other.start_col
        {
            result.push(self.clone());
            return result;
        }

        let mut start_row = self.start_row;
        let mut end_row = self.end_row;

        if self.start_row < other.start_row {
            result.push(MapRegion {
                start_row: self.start_row,
                end_row: other.start_row - 1,
                start_col: self.start_col,
                end_col: self.end_col,
            });
            start_row = other.start_row;
        }

        if self.end_row > other.end_row {
            result.push(MapRegion {
                start_row: other.end_row + 1,
                end_row: self.end_row,
                start_col: self.start_col,
                end_col: self.end_col,
            });
            end_row = other.end_row;
        }

        if self.start_col < other.start_col {
            result.push(MapRegion {
                start_row,
                end_row,
                start_col: self.start_col,
                end_col: other.start_col - 1,
            });
        }

        if self.end_col > other.end_col {
            result.push(MapRegion {
                start_row,
                end_row,
                start_col: other.end_col + 1,
                end_col: self.end_col,
            });
        }

        result
    }

    pub fn intersect(&self, other: &MapRegion) -> Vec<MapRegion> {
        let mut result = Vec::new();

        if self.start_row > other.end_row
            || self.end_row < other.start_row
            || self.start_col > other.end_col
            || self.end_col < other.start_col
        {
            return result;
        }

        result.push(MapRegion {
            start_row: self.start_row.max(other.start_row),
            end_row: self.end_row.min(other.end_row),
            start_col: self.start_col.max(other.start_col),
            end_col: self.end_col.min(other.end_col),
        });

        result
    }
}
