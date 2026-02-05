use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    mut player_query: Single<
        (
            Entity,
            &GameObj,
            &mut MoveComponent,
            &mut WeaponComponent,
            &mut Transform,
        ),
        With<Player>,
    >,
    key_input: Res<ButtonInput<KeyCode>>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    game_lib: Res<GameLib>,
    mut commands: Commands,
    mut exit_app: MessageWriter<AppExit>,
    time: Res<Time>,
) {
    player_query.3.fire_timer.tick(time.delta());

    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if !player_query.3.fire_timer.is_finished() {
            return;
        }

        let base_velocity = player_query.2.speed * player_query.1.direction;

        fire_missiles(
            player_query.1.pos,
            player_query.1.direction,
            player_query.3.missile_config_index,
            &player_query.3.fire_points,
            &player_query.3.fire_directions,
            &base_velocity,
            game_map.as_mut(),
            world_info.as_mut(),
            game_lib.as_ref(),
            &mut commands,
        )
        .unwrap_or_else(|err| {
            error!("Failed to fire missiles: {}", err);
            exit_app.write(AppExit::error());
        });
    } else if key_input.just_pressed(KeyCode::KeyS) {
        player_query.2.speed = 0.0;
    }
}
