use crate::misc::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Play,
    GameOver,
    End,
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub enum GameResult {
    #[default]
    Pending,
    Win,
    Fail,
}

#[derive(Resource, Debug)]
pub struct GameInfo {
    ai_bot_count: usize,
    game_result: GameResult,
}

impl GameInfo {
    pub fn new() -> Self {
        Self {
            ai_bot_count: 0,
            game_result: GameResult::Pending,
        }
    }

    #[inline]
    pub fn ai_bot_count(&self) -> usize {
        self.ai_bot_count
    }

    #[inline]
    pub fn game_result(&self) -> GameResult {
        self.game_result
    }

    #[inline]
    pub fn incr_ai_bot_count(&mut self) {
        self.ai_bot_count += 1;
    }

    pub fn dec_ai_bot_count(&mut self) -> Result<usize, MyError> {
        if self.ai_bot_count >= 1 {
            self.ai_bot_count -= 1;
            Ok(self.ai_bot_count)
        } else {
            let msg = "Try to decrease ai_bot_count while ai_bot_count is zero".to_string();
            error!(msg);
            Err(MyError::Other(msg))
        }
    }

    pub fn set_game_result(&mut self, new_result: GameResult) {
        self.game_result = new_result;
    }
}
