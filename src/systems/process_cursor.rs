use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn process_cursor(
    mut q_player: Single<(Entity, &mut MoveComponent, &mut Transform), With<Player>>,
    mut cursor_reader: MessageReader<CursorMoved>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
) {
    for cursor_moved in cursor_reader.read() {
        let Some(cursor_pos) = translate_cursor_pos(
            cursor_moved.position,
            q_camera.0,
            q_camera.1,
            world_info.as_ref(),
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
}
