use crate::config::{BotConfig, TileConfig};
use crate::game_utils::MapRegion;

pub type BotConfigPair = (String, BotConfig);
pub type TileConfigPair = (String, TileConfig);

#[derive(Debug, Clone)]
pub enum GeneratedMapItem {
    Bot(BotConfigPair),
    Tile(TileConfigPair),
}
pub struct GeneratedMap {
    map: Vec<Vec<Vec<GeneratedMapItem>>>,
}

impl GeneratedMap {
    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            map: vec![vec![vec![]; col_count]; row_count],
        }
    }

    #[inline]
    pub fn get_map(&self) -> &Vec<Vec<Vec<GeneratedMapItem>>> {
        &self.map
    }
}
