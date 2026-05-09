use crate::game_utils::GameInfo;
use crate::misc::GameState;
use bevy::prelude::*;

pub fn check_game(game_info: Res<GameInfo>, mut game_state: ResMut<NextState<GameState>>) {
    if game_info.is_game_over() {
        game_state.set(GameState::GameOver);
    }
}
