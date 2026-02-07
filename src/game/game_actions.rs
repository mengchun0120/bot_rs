use crate::game_utils::*;
use bevy::prelude::*;

pub fn translate_cursor_pos(
    cursor_pos: Vec2,
    camera: &Camera,
    transform: &GlobalTransform,
    world_info: &WorldInfo,
) -> Option<Vec2> {
    let pos = match camera.viewport_to_world_2d(transform, cursor_pos) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to transform cursor position: {}", err);
            return None;
        }
    };

    Some(world_info.viewport_to_world(&pos))
}
