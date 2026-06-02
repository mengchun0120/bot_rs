use crate::config::{BotConfig, TileConfig};
use bevy::prelude::*;

pub type BotConfigPair = (String, BotConfig);
pub type TileConfigPair = (String, TileConfig);

#[derive(Debug, Clone)]
pub enum GeneratedMapItem {
    Bot(BotConfigPair),
    Tile(TileConfigPair),
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

    pub fn add(&mut self, x: f32, y: f32, item: GeneratedMapItem) -> bool {
        let row = self.get_index(y);
        let col = self.get_index(x);
        if !(0..self.col_count()).contains(&row) || !(0..self.row_count()).contains(&col) {
            error!("Cannot add map item {:?} at ({}, {})", item, x, y);
            return false;
        }

        self.map[row][col].push(item);

        true
    }

    #[inline]
    pub fn get_map(&self) -> &Vec<Vec<Vec<GeneratedMapItem>>> {
        &self.map
    }
}
