use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    mut player_query: Single<(Entity, &mut MoveComponent, &mut WeaponComponent), With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    obj_query: Query<&mut GameObj>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    game_lib: Res<GameLib>,
    mut commands: Commands,
    time: Res<Time>,
) {
    player_query.2.fire_timer.tick(time.delta());

    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if !player_query.2.fire_timer.is_finished() {
            return;
        }

        if shoot(
            player_query.0,
            player_query.1.speed,
            player_query.2.as_mut(),
            &obj_query,
            game_map.as_mut(),
            world_info.as_mut(),
            game_lib.as_ref(),
            &mut commands,
        )
        .is_err()
        {
            error!("Failed to shoot");
        }
    } else if key_input.just_pressed(KeyCode::KeyS) {
        player_query.1.speed = 0.0;
    }
}
