use crate::game::{components::*, *};
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut missile_query: Query<(
        Entity,
        &mut Transform,
        &MoveComponent,
        &mut MissileComponent,
    )>,
    mut hp_query: Query<&mut HPComponent>,
    mut world_info: ResMut<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, move_comp, mut missile_comp) in missile_query.iter_mut() {
        let Ok(obj) = game_obj_lib.get(&entity).cloned() else {
            continue;
        };

        if obj.state != GameObjState::Alive {
            continue;
        }

        if let Some(alive_timer) = missile_comp.alive_timer.as_mut() {
            if alive_timer.tick(time.delta()).is_finished() {
                if on_death(
                    entity,
                    &mut hp_query,
                    game_map.as_ref(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    new_obj_queue.as_mut(),
                    &mut commands,
                )
                .is_ok()
                {
                    let _ = despawn_pool.add(entity, game_obj_lib.as_mut());
                }

                continue;
            }
        }

        if let Some(enemy_search_ability) = missile_comp.enemy_search_ability.as_mut() {
            let _ = enemy_search_ability.update(
                &entity,
                transform.as_mut(),
                game_map.as_ref(),
                game_obj_lib.as_mut(),
                time.as_ref(),
            );
        }

        if let Some(pierce_ability) = missile_comp.pierce_ability.as_mut() {
            let _ = pierce_ability.move_obj(
                entity,
                move_comp.speed,
                transform.as_mut(),
                &mut hp_query,
                world_info.as_ref(),
                game_map.as_mut(),
                game_obj_lib.as_mut(),
                game_lib.as_ref(),
                new_obj_queue.as_mut(),
                despawn_pool.as_mut(),
                &mut commands,
                time.as_ref(),
            );
        } else {
            let _ = move_missile(
                entity,
                move_comp.speed,
                transform.as_mut(),
                &mut hp_query,
                world_info.as_mut(),
                game_map.as_mut(),
                game_obj_lib.as_mut(),
                game_lib.as_ref(),
                new_obj_queue.as_mut(),
                despawn_pool.as_mut(),
                &mut commands,
                time.as_ref(),
            );
        }
    }
}
