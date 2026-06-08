use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GameMapConfig {
    pub row_count: usize,
    pub col_count: usize,
    pub objs: Vec<GameMapObjConfig>,
}

#[derive(Deserialize, Serialize)]
pub struct GameMapObjConfig {
    pub config_name: String,
    pub pos: [f32; 2],
    pub direction: [f32; 2],
    pub speed: Option<f32>,
}
