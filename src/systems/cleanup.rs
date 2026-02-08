use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn cleanup(
    mut commands: Commands,
    mut game_map: ResMut<GameMap>,
    obj_query: Query<&GameObj>,
    mut despawn_pool: ResMut<DespawnPool>,
) {
    for entity in despawn_pool.iter() {
        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj {}", entity);
            continue;
        };

        game_map.remove(entity, &obj.map_pos);

        let mut entity_cmd = commands.entity(*entity);
        entity_cmd.despawn_children();
        entity_cmd.despawn();
    }

    despawn_pool.clear();
}
