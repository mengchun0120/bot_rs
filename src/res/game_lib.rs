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
    pub game_obj_configs: HashMap<String, GameObjConfig>,
    pub images: HashMap<String, Handle<Image>>,
}

impl GameLib {
    pub fn load<P: AsRef<Path>>(
        config_path: P,
        asset_server: &AssetServer,
    ) -> Result<Self, MyError> {
        let game_config: GameConfig = read_json(config_path)?;
        let game_obj_configs = read_json(game_config.game_obj_config_file())?;
        let images = Self::load_images(&game_config, asset_server)?;
        let game_lib = GameLib {
            game_config,
            game_obj_configs,
            images,
        };

        info!("GameLib initialized");

        Ok(game_lib)
    }

    pub fn load_images(
        game_config: &GameConfig,
        asset_server: &AssetServer,
    ) -> Result<HashMap<String, Handle<Image>>, MyError> {
        let image_dir = game_config.image_dir();
        let image_configs: HashMap<String, String> = read_json(game_config.image_config_file())?;
        let mut images: HashMap<String, Handle<Image>> = HashMap::new();

        for (name, file_path) in image_configs.iter() {
            if images.contains_key(name) {
                error!("Duplicate image key: {}", name);
                return Err(MyError::DuplicateKey(name.clone()));
            }

            let image_file = image_dir.join(file_path);
            let image = asset_server.load(image_file);
            images.insert(name.clone(), image);
        }

        Ok(images)
    }
}
