use crate::config::{game_config::*, game_map_config::*, game_obj_config::*};
use crate::game::game_obj::*;
use crate::game_utils::{game_lib::*, game_obj_lib::*};
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
    visible_region: MapRegion,
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct MapPos {
    pub row: usize,
    pub col: usize,
}

#[derive(Resource, Eq, PartialEq, Default)]
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

        if !self.is_inside(pos, obj_config.collide_span) {
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
    pub fn is_inside(&self, pos: &Vec2, collide_span: f32) -> bool {
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

    pub fn get_bot_pos_after_collide(
        &self,
        pos: &Vec2,
        direction: &Vec2,
        collide_span: f32,
    ) -> (bool, Vec2) {
        let (collide_bounds, pos) =
            self.get_bot_pos_after_collide_bounds(&pos, &direction, collide_span);

        (collide_bounds, pos)
    }

    pub fn update_origin(
        &mut self,
        origin: &Vec2,
        game_obj_lib: &GameObjLib,
        commands: &mut Commands,
    ) {
        self.origin.x = origin.x.clamp(self.min_origin.x, self.max_origin.x);
        self.origin.y = origin.y.clamp(self.min_origin.y, self.max_origin.y);

        let new_visible_region = self.get_visible_region(&self.origin);
        let update_region = self.visible_region.merge(&new_visible_region);
        self.update_screen_pos(&update_region, game_obj_lib, commands);

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
        self.origin = Vec2::new(
            player_pos.x.clamp(self.min_origin.x, self.max_origin.x),
            player_pos.y.clamp(self.min_origin.y, self.max_origin.y),
        );
    }

    fn setup_visible_region(&mut self, game_config: &GameConfig) {
        self.visible_span.x = game_config.window_width() / 2.0 + game_config.window_ext_size;
        self.visible_span.y = game_config.window_height() / 2.0 + game_config.window_ext_size;
        self.visible_region = self.get_visible_region(&self.origin);
    }

    fn get_visible_region(&self, origin: &Vec2) -> MapRegion {
        MapRegion {
            start_row: self.get_row(origin.y - self.visible_span.y),
            end_row: self.get_row(origin.y + self.visible_span.y),
            start_col: self.get_col(origin.x - self.visible_span.x),
            end_col: self.get_col(origin.x + self.visible_span.x),
        }
    }

    fn get_row(&self, y: f32) -> usize {
        let i = (y / self.cell_size).floor() as i32;
        i.clamp(0, (self.row_count() - 1) as i32) as usize
    }

    fn get_col(&self, x: f32) -> usize {
        let i = (x / self.cell_size).floor() as i32;
        i.clamp(0, (self.col_count() - 1) as i32) as usize
    }

    fn update_screen_pos(
        &self,
        region: &MapRegion,
        game_obj_lib: &GameObjLib,
        commands: &mut Commands,
    ) {
        for row in region.start_row..=region.end_row {
            for col in region.start_col..=region.end_col {
                for entity in self.map[row][col].iter() {
                    let Some(obj) = game_obj_lib.get(entity) else {
                        error!("Cannot find entity in GameObjLib");
                        continue;
                    };
                    let screen_pos = self.get_screen_pos(&obj.pos);

                    commands
                        .entity(entity.clone())
                        .entry::<Transform>()
                        .and_modify(move |mut t| {
                            t.translation.x = screen_pos.x;
                            t.translation.y = screen_pos.y;
                        });
                }
            }
        }
    }

    fn get_bot_pos_after_collide_bounds(
        &self,
        pos: &Vec2,
        direction: &Vec2,
        collide_span: f32,
    ) -> (bool, Vec2) {
        let left = pos.x - collide_span;
        let right = pos.x + collide_span;
        let dx = if left < 0.0 {
            -left
        } else if right > self.width {
            self.width - right
        } else {
            0.0
        };

        let bottom = pos.y - collide_span;
        let top = pos.y + collide_span;
        let dy = if bottom < 0.0 {
            -bottom
        } else if top > self.height {
            self.height - top
        } else {
            0.0
        };

        let mut corrected_pos = pos.clone();
        let min_x = collide_span;
        let max_x = self.width - collide_span;
        let min_y = collide_span;
        let max_y = self.height - collide_span;

        let collide = if dx == 0.0 && dy == 0.0 {
            false
        } else {
            if dx.signum() * direction.x.signum() < 0.0 && dy.signum() * direction.y.signum() < 0.0
            {
                if (dx * direction.y).abs() < (dy * direction.x).abs() {
                    corrected_pos.x = corrected_pos.x.clamp(min_x, max_x);
                    corrected_pos.y += dy.signum() * (dx * direction.y / direction.x).abs();
                    corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
                } else {
                    corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
                    corrected_pos.x += dx.signum() * (dy * direction.x / direction.y).abs();
                    corrected_pos.x = corrected_pos.x.clamp(min_y, max_y);
                }
            } else {
                corrected_pos.x = corrected_pos.x.clamp(min_x, max_x);
                corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
            }
            true
        };

        (collide, corrected_pos)
    }
}

impl MapRegion {
    pub fn merge(&self, other: &MapRegion) -> MapRegion {
        MapRegion {
            start_row: self.start_row.min(other.start_row),
            end_row: self.end_row.max(other.end_row),
            start_col: self.start_col.min(other.start_col),
            end_col: self.end_col.max(other.end_col),
        }
    }
}
