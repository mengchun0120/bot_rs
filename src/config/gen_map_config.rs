use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenMapConfig {
    pub ai_bot_count: usize,
    pub row_count: usize,
    pub col_count: usize,
    pub algorithm: GenMapAlgorithmConfig,
}

#[derive(Debug, Deserialize)]
pub enum GenMapAlgorithmConfig {
    Island(IslandGenMapAlgorithm),
}

#[derive(Debug, Deserialize)]
pub struct IslandGenMapAlgorithm {
    pub min_island_dist: f32,
    pub max_island_dist: f32,
    pub min_island_width: f32,
    pub max_island_width: f32,
    pub min_island_height: f32,
    pub max_island_height: f32,
}
