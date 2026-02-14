use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_player(
    mut player_query: Query<
        (Entity, &mut Transform, &mut Visibility, &MoveComponent),
        With<PlayerComponent>,
    >,
    mut hp_query: Query<&mut HPComponent>,
    mut game_map: ResMut<GameMap>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok((entity, mut transform, mut visibility, move_comp)) = player_query.single_mut() else {
        return;
    };
    if move_bot(
        entity,
        move_comp.speed,
        transform.as_mut(),
        visibility.as_mut(),
        &mut hp_query,
        world_info.as_ref(),
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
        error!("Failed to move player");
        return;
    };
}
