use bevy::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Resource, Deserialize)]
pub struct GameConfig {
    window_size: [f32; 2],
    game_obj_config_file: Vec<String>,
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
    pub fn game_obj_config_file(&self) -> PathBuf {
        self.game_obj_config_file.iter().collect()
    }
}
