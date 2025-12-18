use crate::my_error::*;
use crate::utils::*;
use bevy::prelude::*;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct GameMapConfig {
    pub row_count: usize,
    pub col_count: usize,
    pub objs: Vec<GameMapObjConfig>,
}

#[derive(Deserialize)]
pub struct GameMapObjConfig {
    pub config_name: String,
    pub pos: [f32; 2],
    pub direction: [f32; 2],
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

    pub fn load<P: AsRef<Path>>(map_path: P, cell_size: f32) -> Result<GameMap, MyError> {
        let map_config: GameMapConfig = read_json(map_path)?;
        let map = Self::new(cell_size, map_config.row_count, map_config.col_count);

        Ok(map)
    }
}
