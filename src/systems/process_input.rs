use crate::game::components::*;
use crate::game_utils::{game_map::*, game_obj_lib::*};
use crate::misc::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn process_input(
    mut q_player: Single<(Entity, &mut PlayerComponent, &mut Transform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    game_map: Res<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Some(cursor_pos) = get_cursor_pos(&q_window, &q_camera, game_map.as_ref()) else {
            return;
        };
        let Some(obj) = game_obj_lib.get_mut(&q_player.0) else {
            error!("Cannot find player in GameObjLib");
            return;
        };

        q_player.1.dest = Some(cursor_pos);

        let direction = (cursor_pos - obj.pos).normalize();
        obj.direction = direction.clone();
        q_player.2.rotation = get_rotation(&direction);
    }
}

fn get_cursor_pos(
    q_window: &Single<&Window, With<PrimaryWindow>>,
    q_camera: &Single<(&Camera, &GlobalTransform)>,
    game_map: &GameMap,
) -> Option<Vec2> {
    let camera = q_camera.0;
    let transform = q_camera.1;
    let Some(cursor_pos) = q_window.cursor_position() else {
        return None;
    };
    let pos = match camera.viewport_to_world_2d(transform, cursor_pos) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to transform cursor position: {}", err);
            return None;
        }
    };

    Some(game_map.viewport_to_world(&pos))
}
