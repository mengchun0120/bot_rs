use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut missile_query: Query<(Entity, &mut Transform, &MoveComponent), With<MissileComponent>>,
    mut hp_query: Query<&mut HPComponent>,
    mut enemy_search_query: Query<&mut EnemySearchComponent>,
    mut world_info: ResMut<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, move_comp) in missile_query.iter_mut() {
        let Ok(obj) = game_obj_lib.get(&entity).cloned() else {
            continue;
        };

        if obj.state != GameObjState::Alive {
            continue;
        }

        if let Ok(mut enemy_search_comp) = enemy_search_query.get_mut(entity) {
            let _ = enemy_search_comp.update(
                &entity,
                transform.as_mut(),
                game_map.as_ref(),
                game_obj_lib.as_mut(),
                time.as_ref(),
            );
        }

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
