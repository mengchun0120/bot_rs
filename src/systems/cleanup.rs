use crate::game_utils::*;
use bevy::prelude::*;

pub fn cleanup(
    mut commands: Commands,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut despawn_pool: ResMut<DespawnPool>,
) {
    for entity in despawn_pool.iter() {
        let Some(map_pos) = game_obj_lib.get(entity).map(|obj| obj.map_pos) else {
            error!("Cannot find GameObj {}", entity);
            continue;
        };

        game_map.remove(entity, &map_pos);
        game_obj_lib.remove(entity);

        let mut entity_cmd = commands.entity(*entity);
        entity_cmd.despawn_children();
        entity_cmd.despawn();
    }

    despawn_pool.clear();
}
