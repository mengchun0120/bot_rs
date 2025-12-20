use crate::config::game_config::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ScreenCoord {
    origin: Vec2,
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