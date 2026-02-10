use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    player_query: Single<Entity, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut weapon_comp_query: Query<&mut WeaponComponent>,
    mut world_info: ResMut<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    time: Res<Time>,
) {
    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if try_shoot(
            player_query.entity(),
            &move_comp_query,
            &mut weapon_comp_query,
            world_info.as_mut(),
            game_obj_lib.as_mut(),
            game_lib.as_ref(),
            new_obj_queue.as_mut(),
            time.as_ref(),
        )
        .is_err()
        {
            error!("Failed to shoot");
        }
    } else if key_input.just_pressed(KeyCode::KeyS) {
        if stop_bot(player_query.entity(), &mut move_comp_query).is_err() {
            error!("Failed to stop");
        }
    }
}
