use crate::config::*;
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
    images: HashMap<String, Handle<Image>>,
    gun_configs: HashMap<String, GunConfig>,
    texture_atlas_layouts: HashMap<String, Handle<TextureAtlasLayout>>,
    ai_configs: HashMap<String, AIConfig>,
}

impl GameLib {
    pub fn load<P: AsRef<Path>>(
        config_path: P,
        asset_server: &AssetServer,
        layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Result<Self, MyError> {
        let game_config: GameConfig = read_json(config_path)?;
        let mut game_lib = GameLib {
            game_config,
            images: HashMap::new(),
            game_obj_configs: Vec::new(),
            game_obj_config_indices: HashMap::new(),
            gun_configs: HashMap::new(),
            texture_atlas_layouts: HashMap::new(),
            ai_configs: HashMap::new(),
        };

        game_lib.load_images(asset_server)?;
        game_lib.load_game_obj_configs(layouts)?;
        game_lib.load_gun_configs()?;
        game_lib.load_ai_configs()?;

        info!("GameLib initialized");

        Ok(game_lib)
    }

    #[inline]
    pub fn get_game_obj_config_index(&self, name: &String) -> Result<usize, MyError> {
        match self.game_obj_config_indices.get(name) {
            Some(index) => Ok(index.clone()),
            None => {
                error!("Cannot find GameObjConfig {}", name);
                Err(MyError::NotFound(name.clone()))
            }
        }
    }

    #[inline]
    pub fn get_game_obj_config(&self, index: usize) -> &GameObjConfig {
        &self.game_obj_configs[index]
    }

    #[inline]
    pub fn get_image(&self, name: &String) -> Result<Handle<Image>, MyError> {
        match self.images.get(name) {
            Some(img) => Ok(img.clone()),
            None => {
                error!("Cannot find image {}", name);
                Err(MyError::NotFound(name.clone()))
            }
        }
    }

    #[inline]
    pub fn get_gun_config(&self, name: &String) -> Result<&GunConfig, MyError> {
        match self.gun_configs.get(name) {
            Some(gun_config) => Ok(gun_config),
            None => {
                error!("Cannot find GunConfig {}", name);
                Err(MyError::NotFound(name.clone()))
            }
        }
    }

    #[inline]
    pub fn get_tex_atlas_layout(
        &self,
        name: &String,
    ) -> Result<Handle<TextureAtlasLayout>, MyError> {
        match self.texture_atlas_layouts.get(name) {
            Some(layout) => Ok(layout.clone()),
            None => {
                error!("Cannot find TextureAtlasLayout: {}", name);
                Err(MyError::NotFound(name.clone()))
            }
        }
    }

    #[inline]
    pub fn get_ai_config(&self, name: &String) -> Result<&AIConfig, MyError> {
        match self.ai_configs.get(name) {
            Some(ai_config) => Ok(ai_config),
            None => {
                error!("Cannot find AIConfig {}", name);
                Err(MyError::NotFound(name.clone()))
            }
        }
    }

    fn load_images(&mut self, asset_server: &AssetServer) -> Result<(), MyError> {
        let assets_dir = PathBuf::from("assets");
        let image_dir = self.game_config.image_dir();
        let image_configs: HashMap<String, String> =
            read_json(self.game_config.image_config_file())?;

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

    fn load_game_obj_configs(
        &mut self,
        layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Result<(), MyError> {
        self.game_obj_configs = read_json(self.game_config.game_obj_config_file())?;

        for i in 0..self.game_obj_configs.len() {
            let name = &self.game_obj_configs[i].name;

            if self.game_obj_config_indices.contains_key(name) {
                error!("Duplicate GameObjConfig name {}", name);
                return Err(MyError::DuplicateKey(name.clone()));
            }

            self.game_obj_config_indices.insert(name.clone(), i);

            if let Some(play_config) = self.game_obj_configs[i].play_config.as_ref() {
                let layout = Self::create_tex_atlas_layout(
                    &self.game_obj_configs[i].size,
                    play_config.frame_count,
                    layouts,
                );
                self.texture_atlas_layouts.insert(name.clone(), layout);
            }
        }

        Ok(())
    }

    fn load_gun_configs(&mut self) -> Result<(), MyError> {
        self.gun_configs = read_json(self.game_config.gun_config_file())?;
        Ok(())
    }

    fn load_ai_configs(&mut self) -> Result<(), MyError> {
        self.ai_configs = read_json(self.game_config.ai_config_file())?;
        Ok(())
    }

    fn create_tex_atlas_layout(
        size: &[f32; 2],
        frame_count: usize,
        layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Handle<TextureAtlasLayout> {
        let tile_size = UVec2 {
            x: size[0] as u32,
            y: size[1] as u32,
        };
        layouts.add(TextureAtlasLayout::from_grid(
            tile_size,
            frame_count as u32,
            1,
            None,
            None,
        ))
    }
}
