use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn process_cursor(
    mut player_query: Single<
        (Entity, &mut GameObj, &mut MoveComponent, &mut Transform),
        With<Player>,
    >,
    mut cursor_reader: MessageReader<CursorMoved>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    world_info: Res<WorldInfo>,
) {
    for cursor_moved in cursor_reader.read() {
        let Some(cursor_pos) = translate_cursor_pos(
            cursor_moved.position,
            camera_query.0,
            camera_query.1,
            world_info.as_ref(),
        ) else {
            return;
        };

        let direction = (cursor_pos - player_query.1.pos).normalize();
        player_query.1.direction = direction;
        player_query.3.rotation = get_rotation(&direction);
    }
}
