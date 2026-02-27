use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn cleanup(
    mut commands: Commands,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_info: ResMut<GameInfo>,
) {
    for entity in despawn_pool.iter() {
        let Ok(obj) = game_obj_lib.get(entity).cloned() else {
            continue;
        };

        game_map.remove(entity, &obj.map_pos);
        game_obj_lib.remove(entity);

        let mut entity_cmd = commands.entity(*entity);
        entity_cmd.despawn_children();
        entity_cmd.despawn();

        if obj.is_player() {
            game_state.set(GameState::GameOver);
            game_info.set_game_result(GameResult::Fail);
            info!("Game over: player fails");
        } else if obj.is_ai_bot() {
            if let Ok(()) = game_info.dec_ai_bot_count()
                && game_info.ai_bot_count() == 0
            {
                game_state.set(GameState::GameOver);
                game_info.set_game_result(GameResult::Win);
                info!("Game over: player wins");
            }
        }
    }

    despawn_pool.clear();
}
