use crate::game::{components::*, game_actions::*};
use crate::game_utils::*;
use crate::misc::collide::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut q_missile: Query<(Entity, &mut Transform), With<MissileComponent>>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    time: Res<Time>,
) {
    for (entity, mut transform) in q_missile.iter_mut() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get(&entity).cloned() else {
            error!("Cannot find entity in GameObjLib");
            continue;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);
        let new_pos = obj.pos + obj.direction * time.delta_secs() * obj_config.speed;

        if !game_map.check_pos_visible(&new_pos) {
            despawn_pool.insert(entity);
            continue;
        }

        if check_missile_collide(
            &new_pos,
            obj_config.collide_span,
            obj_config.side,
            game_map.as_ref(),
            game_obj_lib.as_ref(),
            game_lib.as_ref(),
            despawn_pool.as_ref(),
        ) {
            despawn_pool.insert(entity);
            continue;
        }

        update_obj_pos(
            entity,
            &new_pos,
            game_obj_lib.as_mut(),
            game_map.as_mut(),
            transform.as_mut(),
        );
    }
}
