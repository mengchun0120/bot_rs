use crate::game_utils::{game_lib::*, game_map::*, game_obj_lib::*};
use crate::misc::my_error::*;
use bevy::prelude::*;

pub fn fire_missiles(
    entity: Entity,
    missile_config_index: usize,
    fire_points: &Vec<Vec2>,
    fire_directions: &Vec<Vec2>,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        error!("Cannot find entity in GameObjLib");
        return Err(MyError::NotFound("entity".into()));
    };

    for i in 0..fire_points.len() {
        let direction = obj.direction.rotate(fire_directions[i]);
        let pos = obj.pos + direction.rotate(fire_points[i]);

        if !game_map.check_pos_visible(&pos) {
            continue;
        }

        game_map.add_obj_by_index(
            missile_config_index,
            &pos,
            &direction,
            game_lib,
            game_obj_lib,
            commands,
        )?;
    }

    Ok(())
}
