use crate::config::{GameMapConfig, GameMapObjConfig, NamedGameObjConfig};
use crate::game_utils::MapRegion;
use crate::misc::MyError;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct GeneratedMapItem {
    pub pos: Vec2,
    pub direction: Vec2,
    pub config: NamedGameObjConfig,
}

pub struct GeneratedMap {
    cell_size: f32,
    map: Vec<Vec<Vec<GeneratedMapItem>>>,
}

impl GeneratedMap {
    pub fn new(row_count: usize, col_count: usize, cell_size: f32) -> Self {
        Self {
            cell_size,
            map: vec![vec![vec![]; col_count]; row_count],
        }
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
    pub fn width(&self) -> f32 {
        self.col_count() as f32 * self.cell_size
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.row_count() as f32 * self.cell_size
    }

    #[inline]
    pub fn get_index(&self, c: f32) -> usize {
        (c / self.cell_size).floor() as usize
    }

    #[inline]
    pub fn get_cell(&self, row: usize, col: usize) -> &Vec<GeneratedMapItem> {
        &self.map[row][col]
    }

    #[inline]
    pub fn get_map_region(
        &self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> Result<MapRegion, MyError> {
        let height = self.height();
        let bottom = bottom.clamp(0.0, height);
        let top = top.clamp(0.0, height);

        if bottom > top {
            let msg = "Invalid parameters: bottom must be no greater than top".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        }

        let width = self.width();
        let left = left.clamp(0.0, width);
        let right = right.clamp(0.0, width);

        if left > right {
            let msg = "Invalid parameters: left must be no greater than right".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        }

        let max_row = self.row_count() - 1;
        let max_col = self.col_count() - 1;

        let region = MapRegion {
            start_row: self.get_index(bottom).clamp(0, max_row),
            end_row: self.get_index(top).clamp(0, max_row),
            start_col: self.get_index(left).clamp(0, max_col),
            end_col: self.get_index(right).clamp(0, max_col),
        };

        Ok(region)
    }

    pub fn add(
        &mut self,
        pos: Vec2,
        direction: Vec2,
        config: NamedGameObjConfig,
    ) -> Result<(), MyError> {
        if pos.x < 0.0 || pos.x >= self.width() || pos.y < 0.0 || pos.y >= self.height() {
            let msg = format!("pos={} is out of range", pos);
            error!(msg);
            return Err(MyError::Other(msg));
        }

        let row = self.get_index(pos.y);
        let col = self.get_index(pos.x);

        info!(
            "Added obj {:?} with pos={} direction={} at ({}, {})",
            config, pos, direction, row, col
        );

        self.map[row][col].push(GeneratedMapItem {
            pos,
            direction,
            config,
        });

        Ok(())
    }

    pub fn to_map_config(&self) -> GameMapConfig {
        let mut objs: Vec<GameMapObjConfig> = Vec::new();

        for row in self.map.iter() {
            for cell in row.iter() {
                for item in cell.iter() {
                    objs.push(GameMapObjConfig {
                        config_name: item.config.name.clone(),
                        pos: [item.pos.x, item.pos.y],
                        direction: [item.direction.x, item.direction.y],
                        speed: None,
                    });
                }
            }
        }

        let map_config = GameMapConfig {
            row_count: self.row_count(),
            col_count: self.col_count(),
            objs,
        };

        map_config
    }
}
