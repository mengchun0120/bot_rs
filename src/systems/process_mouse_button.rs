use crate::game::*;
use crate::game_utils::*;
use crate::misc::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn process_mouse_button(
    mut player_query: Single<(Entity, &mut MoveComponent, &mut Transform), With<Player>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Some(obj) = game_obj_lib.get_mut(&player_query.0) else {
            error!("Cannot find Player");
            return;
        };
        let Some(cursor_pos) = window_query.cursor_position() else {
            warn!("Failed to get cursor position");
            return;
        };
        let Some(cursor_pos) = translate_cursor_pos(
            cursor_pos,
            camera_query.0,
            camera_query.1,
            world_info.as_ref(),
        ) else {
            return;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);
        let direction = (cursor_pos - obj.pos).normalize();

        obj.direction = direction;
        player_query.1.speed = obj_config.speed;
        player_query.2.rotation = get_rotation(&direction);
    }
}
