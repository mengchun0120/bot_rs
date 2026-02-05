use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut q_missile: Query<
        (Entity, &MoveComponent, &mut Transform),
        With<MissileComponent>,
    >,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut obj_query: Query<&mut GameObj>,
    mut hp_query: Query<&mut HPComponent>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, move_comp, mut transform) in q_missile.iter_mut() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Ok(obj) = obj_query.get(entity).cloned() else {
            continue;
        };

        let obj_config = game_lib.get_game_obj_config(obj.config_index);
        let new_pos = obj.pos + obj.direction * move_comp.speed * time.delta_secs();

        if !world_info.check_pos_visible(&new_pos) {
            despawn_pool.insert(entity);
            continue;
        }

        if check_collide(
            &entity,
            &new_pos,
            obj_config.collide_span,
            game_map.as_ref(),
            world_info.as_ref(),
            QueryMapperByMut::new(&obj_query),
            game_lib.as_ref(),
            despawn_pool.as_ref(),
        ) {
            if let Some(explosion) = obj_config.explosion.as_ref() {
                if explode(
                    explosion,
                    new_pos,
                    game_map.as_mut(),
                    world_info.as_mut(),
                    &QueryMapperByMut::new(&obj_query),
                    &mut MutQueryMapper::new(&mut hp_query),
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
        } else {
            obj_query.get_mut(entity).and_then(|mut obj| {
                update_obj_pos(
                    entity,
                    obj.as_mut(),
                    new_pos,
                    game_map.as_mut(),
                    world_info.as_ref(),
                    transform.as_mut(),
                );
                Ok(())
            });



        }
    }
}
