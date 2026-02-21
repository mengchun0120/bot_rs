use crate::misc::*;
use crate::systems::game_play::*;
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), setup_game)
        .add_systems(PreUpdate, update_ai.run_if(in_state(AppState::Game)))
        .add_systems(
            Update,
            (
                process_cursor,
                process_key,
                process_mouse_button,
                update_ai_bots,
                update_player,
                update_missiles,
                update_playout,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(
            PostUpdate,
            (update_origin, cleanup, add_new_objs)
                .chain()
                .run_if(in_state(AppState::Game)),
        );
}
