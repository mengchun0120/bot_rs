use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize)]
pub struct GameConfig {
    window_size: [f32; 2],
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
}
