use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn add_new_objs(
    world_info: Res<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
) {
    for new_obj in new_obj_queue.iter() {
        if create_obj_by_index(
            new_obj.config_index,
            new_obj.pos,
            new_obj.direction,
            new_obj.speed,
            world_info.as_ref(),
            game_map.as_mut(),
            game_obj_lib.as_mut(),
            game_lib.as_ref(),
            &mut commands,
            game_info.as_mut(),
        )
        .is_err()
        {
            error!("Failed to create object");
        }
    }

    new_obj_queue.clear();
}
