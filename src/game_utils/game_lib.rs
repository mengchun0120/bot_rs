use crate::config::{game_config::*, game_obj_config::*};
use crate::game::game_obj::*;
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Resource)]
pub struct GameLib {
    pub game_config: GameConfig,
    pub game_obj_configs: HashMap<String, GameObjConfig>,
    pub images: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameObjLib(pub HashMap<Entity, GameObj>);

#[derive(Resource)]
pub struct ScreenCoord {
    origin: Vec2,
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
        let assets_dir = PathBuf::from("assets");
        let image_dir = game_config.image_dir();
        let image_configs: HashMap<String, String> = read_json(game_config.image_config_file())?;
        let mut images: HashMap<String, Handle<Image>> = HashMap::new();

        for (name, file_path) in image_configs.iter() {
            if images.contains_key(name) {
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
            images.insert(name.clone(), image);
        }

        Ok(images)
    }
}

impl GameObjLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl ScreenCoord {
    pub fn new(game_config: &GameConfig) -> Self {
        Self {
            origin: Vec2::new(
                game_config.window_width() / 2.0,
                game_config.window_height() / 2.0,
            ),
        }
    }

    #[inline]
    pub fn screen_pos(&self, pos: &Vec2) -> Vec2 {
        pos - self.origin
    }
}
