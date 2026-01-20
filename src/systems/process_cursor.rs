use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn process_cursor(
    mut q_player: Single<(Entity, &mut PlayerComponent, &mut Transform)>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_map: Res<GameMap>,
) {
    if q_player.1.move_enabled() {
        return;
    }

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

    let direction = (cursor_pos - obj.pos).normalize();
    obj.direction = direction.clone();
    q_player.2.rotation = get_rotation(&direction);
}
