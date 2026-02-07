use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_missiles(
    q_missile: Query<(Entity, &MoveComponent), With<MissileComponent>>,
    mut obj_query: Query<&mut GameObj>,
    mut transform_query: Query<&mut Transform>,
    mut hp_query: Query<&mut HPComponent>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, move_comp) in q_missile.iter() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        if move_missile(
            entity,
            move_comp,
            &mut obj_query,
            &mut transform_query,
            &mut hp_query,
            game_map.as_mut(),
            world_info.as_mut(),
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
            time.as_ref(),
        )
        .is_err()
        {
            error!("Cannot move missile");
        }
    }
}
