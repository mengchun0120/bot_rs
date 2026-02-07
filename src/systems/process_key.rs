use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    player_query: Single<Entity, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    obj_query: Query<&mut GameObj>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut weapon_comp_query: Query<&mut WeaponComponent>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    game_lib: Res<GameLib>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if try_shoot(
            player_query.entity(),
            &move_comp_query,
            &mut weapon_comp_query,
            &obj_query,
            game_map.as_mut(),
            world_info.as_mut(),
            game_lib.as_ref(),
            &mut commands,
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
