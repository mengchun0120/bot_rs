use crate::config::{game_config::*, game_obj_config::*};
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Resource)]
pub struct GameLib {
    pub game_config: GameConfig,
    game_obj_configs: Vec<GameObjConfig>,
    game_obj_config_indices: HashMap<String, usize>,
    pub images: HashMap<String, Handle<Image>>,
}

impl GameLib {
    pub fn load<P: AsRef<Path>>(
        config_path: P,
        asset_server: &AssetServer,
    ) -> Result<Self, MyError> {
        let game_config: GameConfig = read_json(config_path)?;
        let mut game_lib = GameLib {
            game_config,
            images: HashMap::new(),
            game_obj_configs: Vec::new(),
            game_obj_config_indices: HashMap::new(),
        };

        game_lib.load_images(asset_server)?;
        game_lib.load_game_obj_configs()?;

        info!("GameLib initialized");

        Ok(game_lib)
    }

    #[inline]
    pub fn get_game_obj_config_index(&self, name: &String) -> Option<usize> {
        self.game_obj_config_indices.get(name).copied()
    }

    #[inline]
    pub fn get_game_obj_config(&self, index: usize) -> &GameObjConfig {
        &self.game_obj_configs[index]
    }

    fn load_images(
        &mut self,
        asset_server: &AssetServer,
    ) -> Result<(), MyError> {
        let assets_dir = PathBuf::from("assets");
        let image_dir = self.game_config.image_dir();
        let image_configs: HashMap<String, String> = read_json(self.game_config.image_config_file())?;

        for (name, file_path) in image_configs.iter() {
            if self.images.contains_key(name) {
                error!("Duplicate image key: {}", name);
                return Err(MyError::DuplicateKey(name.clone()));
            }

            let image_relative_path = image_dir.join(file_path);
            let image_abs_path = assets_dir.join(image_relative_path.clone());
            if !image_abs_path.exists() {
                let err_msg = format!("File {:?} doesn't exist", image_abs_path);
                error!(err_msg);
                return Err(Error::new(ErrorKind::NotFound, err_msg).into());
            }

            let image = asset_server.load(image_relative_path);
            self.images.insert(name.clone(), image);
        }

        Ok(())
    }

    fn load_game_obj_configs(&mut self) -> Result<(), MyError> {
        self.game_obj_configs = read_json(self.game_config.game_obj_config_file())?;

        for i in 0..self.game_obj_configs.len() {
            let name = &self.game_obj_configs[i].name;

            if self.game_obj_config_indices.contains_key(name) {
                error!("Duplicate GameObjConfig name {}", name);
                return Err(MyError::DuplicateKey(name.clone()));
            }

            self.game_obj_config_indices.insert(name.clone(), i);
        }

        Ok(())
    }
}
