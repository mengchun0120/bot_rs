use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn process_cursor(
    mut player_query: Single<(Entity, &mut Transform), With<PlayerComponent>>,
    mut cursor_reader: MessageReader<CursorMoved>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
) {
    let Ok(obj) = game_obj_lib.get_mut(&player_query.0) else {
        return;
    };

    if obj.state != GameObjState::Alive {
        return;
    }

    for cursor_moved in cursor_reader.read() {
        let Some(cursor_pos) = translate_cursor_pos(
            cursor_moved.position,
            camera_query.0,
            camera_query.1,
            world_info.as_ref(),
        ) else {
            return;
        };

        let direction = (cursor_pos - obj.pos).normalize();
        obj.direction = direction;
        player_query.1.rotation = get_rotation(&direction);
    }
}
