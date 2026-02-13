use bevy::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Resource, Deserialize)]
pub struct GameConfig {
    window_size: [f32; 2],
    config_dir: Vec<String>,
    map_dir: Vec<String>,
    image_dir: Vec<String>,
    game_obj_config_file: String,
    image_config_file: String,
    gun_config_file: String,
    explosion_config_file: String,
    ai_config_file: String,
    pub cell_size: f32,
    pub window_ext_size: f32,
    pub max_collide_span: f32,
}

impl GameConfig {
    #[inline]
    pub fn window_width(&self) -> f32 {
        self.window_size[0]
    }

    #[inline]
    pub fn window_height(&self) -> f32 {
        self.window_size[1]
    }

    #[inline]
    pub fn config_dir(&self) -> PathBuf {
        self.config_dir.iter().collect()
    }

    #[inline]
    pub fn map_dir(&self) -> PathBuf {
        self.map_dir.iter().collect()
    }

    #[inline]
    pub fn image_dir(&self) -> PathBuf {
        self.image_dir.iter().collect()
    }

    #[inline]
    pub fn game_obj_config_file(&self) -> PathBuf {
        self.config_dir().join(&self.game_obj_config_file)
    }

    #[inline]
    pub fn image_config_file(&self) -> PathBuf {
        self.config_dir().join(&self.image_config_file)
    }

    #[inline]
    pub fn gun_config_file(&self) -> PathBuf {
        self.config_dir().join(&self.gun_config_file)
    }

    #[inline]
    pub fn explosion_config_file(&self) -> PathBuf {
        self.config_dir().join(&self.explosion_config_file)
    }

    #[inline]
    pub fn ai_config_file(&self) -> PathBuf {
        self.config_dir().join(&self.ai_config_file)
    }
}
