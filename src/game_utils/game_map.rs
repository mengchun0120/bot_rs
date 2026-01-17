use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::{my_error::*, utils::*};
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
        }
    }

    pub fn load<P: AsRef<Path>>(
        map_path: P,
        cell_size: f32,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Result<GameMap, MyError> {
        let map_config: GameMapConfig = read_json(map_path)?;
        let mut map = Self::new(cell_size, map_config.row_count, map_config.col_count);

        map.setup_min_max_origin(&game_lib.game_config);
        map.setup_visible_span(&game_lib.game_config);

        let player_pos = arr_to_vec2(&map_config.player.pos);
        map.set_origin(&player_pos);

        map.add_obj_by_config(
            &map_config.player,
            game_lib,
            game_obj_lib,
            commands,
            asset_server,
        )?;

        for map_obj_config in map_config.objs.iter() {
            map.add_obj_by_config(
                map_obj_config,
                game_lib,
                game_obj_lib,
                commands,
                asset_server,
            )?;
        }

        Ok(map)
    }

    pub fn add_obj_by_config(
        &mut self,
        map_obj_config: &GameMapObjConfig,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        commands: &mut Commands,
        asset_server: &AssetServer,
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
            asset_server,
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
        asset_server: &AssetServer,
    ) -> Result<(), MyError> {
        let obj_config = game_lib.get_game_obj_config(config_index);

        if !self.contains(pos, obj_config.collide_span) {
            let err_msg = format!("Position {:?} is outside of map", pos);
            error!(err_msg);
            return Err(MyError::Other(err_msg));
        }

        let (obj, entity) = GameObj::new(
            config_index,
            pos,
            direction,
            self,
            game_lib,
            commands,
            asset_server,
        )?;

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

    pub fn set_origin(&mut self, origin: &Vec2) {
        self.origin.x = origin.x.clamp(self.min_origin.x, self.max_origin.x);
        self.origin.y = origin.y.clamp(self.min_origin.y, self.max_origin.y);
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

    #[inline]
    pub fn get_visible_region(&self) -> MapRegion {
        self.get_region(
            self.origin.x - self.visible_span.x,
            self.origin.y - self.visible_span.y,
            self.origin.x + self.visible_span.x,
            self.origin.y + self.visible_span.y,
        )
    }

    #[inline]
    pub fn get_row(&self, y: f32) -> usize {
        let i = (y / self.cell_size).floor() as i32;
        i.clamp(0, (self.row_count() - 1) as i32) as usize
    }

    #[inline]
    pub fn get_col(&self, x: f32) -> usize {
        let i = (x / self.cell_size).floor() as i32;
        i.clamp(0, (self.col_count() - 1) as i32) as usize
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
    pub fn get_collide_region_bot(
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

    fn setup_min_max_origin(&mut self, game_config: &GameConfig) {
        self.min_origin = Vec2::new(
            game_config.window_width() / 2.0,
            game_config.window_height() / 2.0,
        );
        self.max_origin = Vec2::new(
            self.width - self.min_origin.x,
            self.height - self.min_origin.y,
        );
    }

    fn setup_visible_span(&mut self, game_config: &GameConfig) {
        self.visible_span.x = game_config.window_width() / 2.0 + game_config.window_ext_size;
        self.visible_span.y = game_config.window_height() / 2.0 + game_config.window_ext_size;
    }
}
