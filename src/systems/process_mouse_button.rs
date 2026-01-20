use std::time::Duration;

use crate::game::*;
use crate::game_utils::*;
use crate::misc::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn process_mouse_button(
    mut q_player: Single<(Entity, &mut PlayerComponent, &mut Transform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    game_map: Res<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Some(cursor_pos) = get_cursor_pos(
            q_window.into_inner(),
            q_camera.0,
            q_camera.1,
            game_map.as_ref(),
        ) else {
            return;
        };
        let Some(obj) = game_obj_lib.get_mut(&q_player.0) else {
            error!("Cannot find player in GameObjLib");
            return;
        };

        let move_duration = get_player_move_time(&obj.pos, &cursor_pos, obj, game_lib.as_ref());
        q_player
            .1
            .start_moving(Duration::from_secs_f32(move_duration));

        let direction = (cursor_pos - obj.pos).normalize();
        obj.direction = direction.clone();
        q_player.2.rotation = get_rotation(&direction);
    }
}

fn get_player_move_time(
    start_pos: &Vec2,
    end_pos: &Vec2,
    obj: &GameObj,
    game_lib: &GameLib,
) -> f32 {
    let speed = game_lib.get_game_obj_config(obj.config_index).speed;
    start_pos.distance(end_pos.clone()) / speed
}
