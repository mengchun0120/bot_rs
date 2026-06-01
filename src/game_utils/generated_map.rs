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
    player_pos: Option<(usize, usize)>,
    map: Vec<Vec<Vec<GeneratedMapItem>>>,
}

impl GeneratedMap {
    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            player_pos: None,
            map: vec![vec![vec![]; col_count]; row_count],
        }
    }

    pub fn add(&mut self, row: usize, col: usize, item: GeneratedMapItem) {
        self.map[row][col].push(item);
    }

    pub fn set_player_pos(&mut self, row_index: usize, col_index: usize) {
        self.player_pos = Some((row_index, col_index));
    }

    #[inline]
    pub fn get_player_pos(&self) -> Option<(usize, usize)> {
        self.player_pos.clone()
    }

    #[inline]
    pub fn get_map(&self) -> &Vec<Vec<Vec<GeneratedMapItem>>> {
        &self.map
    }
}
