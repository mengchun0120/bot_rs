use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_missiles(
    q_missile: Query<Entity, With<MissileComponent>>,
    move_comp_query: Query<&mut MoveComponent>,
    mut transform_query: Query<&mut Transform>,
    mut hp_query: Query<&mut HPComponent>,
    mut world_info: ResMut<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    time: Res<Time>,
) {
    for entity in q_missile.iter() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        if move_missile(
            entity,
            &move_comp_query,
            &mut transform_query,
            &mut hp_query,
            world_info.as_mut(),
            game_map.as_mut(),
            game_obj_lib.as_mut(),
            game_lib.as_ref(),
            new_obj_queue.as_mut(),
            despawn_pool.as_mut(),
            time.as_ref(),
        )
        .is_err()
        {
            error!("Cannot move missile");
        }
    }
}
