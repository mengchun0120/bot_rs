use crate::game::*;
use crate::game_utils::*;
use crate::misc::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn process_mouse_button(
    mut q_player: Single<(Entity, &mut MoveComponent, &mut Transform), With<Player>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Some(cursor_pos) = q_window.cursor_position() else {
            warn!("Failed to get cursor position");
            return;
        };

        let Some(cursor_pos) =
            translate_cursor_pos(cursor_pos, q_camera.0, q_camera.1, world_info.as_ref())
        else {
            return;
        };
        let Some(obj) = game_obj_lib.get_mut(&q_player.0) else {
            error!("Cannot find player in GameObjLib");
            return;
        };
        let speed = game_lib.get_game_obj_config(obj.config_index).speed;

        q_player.1.speed = speed;

        let direction = (cursor_pos - obj.pos).normalize();
        obj.direction = direction.clone();
        q_player.2.rotation = get_rotation(&direction);
    }
}
