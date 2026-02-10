use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_player(
    player_query: Single<Entity, With<Player>>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut hp_query: Query<&mut HPComponent>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if move_bot(
        player_query.entity(),
        &mut move_comp_query,
        &mut transform_query,
        &mut visibility_query,
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
        error!("Failed to move player");
        return;
    };
}
