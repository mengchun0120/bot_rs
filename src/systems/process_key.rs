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
        With<Player>,
    >,
    key_input: Res<ButtonInput<KeyCode>>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    obj_query: &Query<&mut GameObj>,
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

        let Some(direction) = game_obj_lib
            .get(&q_player.0)
            .map(|obj| obj.direction.clone())
        else {
            error!("Cannot find player in GameObjLib");
            return;
        };
        let base_velocity = q_player.1.speed * direction;

        fire_missiles(
            q_player.0,
            q_player.2.missile_config_index,
            &q_player.2.fire_points,
            &q_player.2.fire_directions,
            &base_velocity,
            game_map.as_mut(),
            world_info.as_mut(),
            game_obj_lib.as_mut(),
            game_lib.as_ref(),
            &mut commands,
        )
        .unwrap_or_else(|err| {
            error!("Failed to fire missiles: {}", err);
            exit_app.write(AppExit::error());
        });
    } else if key_input.just_pressed(KeyCode::KeyS) {
        q_player.1.speed = 0.0;
    }
}
