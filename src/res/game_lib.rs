use crate::my_error::*;
use crate::res::game_config::*;
use crate::res::game_obj_config::*;
use crate::utils::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path;

#[derive(Resource)]
pub struct GameLib {
    pub game_config: GameConfig,
    pub game_obj_config_map: HashMap<String, GameObjConfig>,
}

impl GameLib {
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self, MyError> {
        let game_config: GameConfig = read_json(config_path)?;
        let game_obj_config_map = read_json(game_config.game_obj_config_file())?;
        let game_lib = GameLib {
            game_config,
            game_obj_config_map,
        };

        info!("GameLib initialized");

        Ok(game_lib)
    }
}
