use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    mut q_player: Single<
        (
            Entity,
            &mut MoveComponent,
            &mut WeaponComponent,
            &mut Transform,
        ),
        With<PlayerComponent>,
    >,
    key_input: Res<ButtonInput<KeyCode>>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut commands: Commands,
    mut exit_app: MessageWriter<AppExit>,
    time: Res<Time>,
) {
    q_player.2.fire_timer.tick(time.delta());

    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if !q_player.2.fire_timer.is_finished() {
            return;
        }

        fire_missiles(
            q_player.0,
            q_player.2.missile_config_index,
            &q_player.2.fire_points,
            &q_player.2.fire_directions,
            game_map.as_mut(),
            game_obj_lib.as_mut(),
            game_lib.as_ref(),
            &mut commands,
        )
        .unwrap_or_else(|err| {
            error!("Failed to fire missiles: {}", err);
            exit_app.write(AppExit::error());
        });
    } else if key_input.just_pressed(KeyCode::KeyS) {
        q_player.1.move_enabled = false;
    }
}
