use crate::config::game_map_config::*;
use crate::game::game_obj::*;
use crate::game_utils::{game_lib::*, screen_coord::*, game_obj_lib::*};
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
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct MapPos {
    pub row: usize,
    pub col: usize,
}

impl GameMap {
    pub fn new(cell_size: f32, row_count: usize, col_count: usize) -> Self {
        Self {
            cell_size,
            width: col_count as f32 * cell_size,
            height: row_count as f32 * cell_size,
            map: vec![vec![HashSet::new(); col_count]; row_count],
            max_collide_span: 0.0,
        }
    }

    pub fn load<P: AsRef<Path>>(
        map_path: P,
        cell_size: f32,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        screen_coord: &ScreenCoord,
        commands: &mut Commands,
    ) -> Result<GameMap, MyError> {
        let map_config: GameMapConfig = read_json(map_path)?;
        let mut map = Self::new(cell_size, map_config.row_count, map_config.col_count);

        for map_obj_config in map_config.objs.iter() {
            let Some(config_index) = game_lib.get_game_obj_config_index(&map_obj_config.config_name) else {
                error!("Cannot find GameObjConfig {}", map_obj_config.config_name);
                return Err(MyError::NotFound(map_obj_config.config_name.clone()));
            };
            let pos = arr_to_vec2(&map_obj_config.pos);
            let direction = arr_to_vec2(&map_obj_config.direction).normalize();

            map.add_obj(
                config_index,
                &pos,
                &direction,
                game_lib,
                game_obj_lib,
                screen_coord,
                commands,
            )?;
        }

        Ok(map)
    }

    pub fn add_obj(
        &mut self,
        config_index: usize,
        pos: &Vec2,
        direction: &Vec2,
        game_lib: &GameLib,
        game_obj_lib: &mut GameObjLib,
        screen_coord: &ScreenCoord,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let obj_config = game_lib.get_game_obj_config(config_index);

        if !self.is_inside(pos, obj_config.collide_span) {
            let err_msg = format!("Position {:?} is outside of map", pos);
            error!(err_msg);
            return Err(MyError::Other(err_msg));
        }

        let map_pos = self.get_map_pos(pos);
        let (obj, entity) = GameObj::new(
            config_index,
            pos,
            &map_pos,
            direction,
            game_lib,
            screen_coord,
            commands,
        )?;

        self.map[map_pos.row][map_pos.col].insert(entity);
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
}
