use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_missiles(
    mut missile_query: Query<(Entity, &mut Transform, &MoveComponent), With<MissileComponent>>,
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
    for (entity, mut transform, move_comp) in missile_query.iter_mut() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        if move_missile(
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
        )
        .is_err()
        {
            error!("Cannot move missile");
        }
    }
}

fn search_enemy(
    entity: &Entity,
    search_span: f32,
    game_map: &GameMap,
    mut game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
) {
    let Ok(obj) = game_obj_lib.get_mut(entity) else {
        return;
    };
}
