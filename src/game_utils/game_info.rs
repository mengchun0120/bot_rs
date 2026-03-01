use crate::misc::*;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GameInfo {
    ai_bot_count: usize,
    player: Option<Entity>,
}

impl GameInfo {
    pub fn new() -> Self {
        Self {
            ai_bot_count: 0,
            player: None,
        }
    }

    #[inline]
    pub fn ai_bot_count(&self) -> usize {
        self.ai_bot_count
    }

    #[inline]
    pub fn incr_ai_bot_count(&mut self) {
        self.ai_bot_count += 1;
    }

    pub fn dec_ai_bot_count(&mut self) -> Result<(), MyError> {
        if self.ai_bot_count >= 1 {
            self.ai_bot_count -= 1;
            Ok(())
        } else {
            let msg = "Try to decrease ai_bot_count while ai_bot_count is zero".to_string();
            error!(msg);
            Err(MyError::Other(msg))
        }
    }

    #[inline]
    pub fn get_player(&self) -> Option<Entity> {
        self.player
    }

    #[inline]
    pub fn set_player(&mut self, entity: Entity) {
        self.player = Some(entity);
    }

    #[inline]
    pub fn clear_player(&mut self) {
        self.player = None;
    }

    #[inline]
    pub fn is_game_over(&self) -> bool {
        self.ai_bot_count == 0 || self.player.is_none()
    }
}
