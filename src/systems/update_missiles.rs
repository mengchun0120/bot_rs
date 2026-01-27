use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut q_missile: Query<(Entity, &MoveComponent, &mut Transform), With<MissileComponent>>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, move_comp, mut transform) in q_missile.iter_mut() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get(&entity).cloned() else {
            error!("Cannot find entity in GameObjLib");
            continue;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        let (collide, time_delta) = check_collide(
            &entity,
            &obj.pos,
            &obj.direction,
            move_comp.speed,
            obj_config,
            game_map.as_ref(),
            game_obj_lib.as_ref(),
            world_info.as_ref(),
            game_lib.as_ref(),
            despawn_pool.as_ref(),
            time.delta_secs(),
        );
        let new_pos = obj.pos + obj.direction * move_comp.speed * time_delta;

        if !world_info.check_pos_visible(&new_pos) {
            despawn_pool.insert(entity);
            continue;
        }

        if collide {
            if let Some(explosion) = obj_config.explosion.as_ref() {
                if explode(
                    explosion,
                    &new_pos,
                    game_obj_lib.as_mut(),
                    game_map.as_mut(),
                    world_info.as_mut(),
                    game_lib.as_ref(),
                    despawn_pool.as_mut(),
                    &mut commands,
                )
                .is_err()
                {
                    error!("Failed to create explosion {}", explosion);
                }
            }
            despawn_pool.insert(entity);
            continue;
        }

        update_obj_pos(
            entity,
            &new_pos,
            game_map.as_mut(),
            world_info.as_ref(),
            game_obj_lib.as_mut(),
            transform.as_mut(),
        );
    }
}
