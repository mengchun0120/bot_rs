use crate::my_error::*;
use crate::res::config::*;
use crate::utils::*;
use bevy::prelude::*;
use std::path::Path;

#[derive(Resource)]
pub struct GameLib {
    pub game_config: GameConfig,
}

impl GameLib {
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self, MyError> {
        let game_config: GameConfig = read_json(config_path)?;
        let game_lib = GameLib { game_config };

        info!("GameLib initialized");

        Ok(game_lib)
    }
}
