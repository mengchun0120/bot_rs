use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn cleanup(
    mut commands: Commands,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for entity in despawn_pool.iter() {
        let Ok(obj) = game_obj_lib.get(entity).cloned() else {
            continue;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        game_map.remove(entity, &obj.map_pos);
        game_obj_lib.remove(entity);

        let mut entity_cmd = commands.entity(*entity);
        entity_cmd.despawn_children();
        entity_cmd.despawn();

        if obj_config.is_player() {
            game_state.set(GameState::GameOver);
            info!("Game over");
        }
    }

    despawn_pool.clear();
}
