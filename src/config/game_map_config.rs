use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameMapConfig {
    pub row_count: usize,
    pub col_count: usize,
    pub player: GameMapObjConfig,
    pub objs: Vec<GameMapObjConfig>,
}

#[derive(Deserialize)]
pub struct GameMapObjConfig {
    pub config_name: String,
    pub pos: [f32; 2],
    pub direction: [f32; 2],
}
