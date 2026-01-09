use crate::game_utils::{despawn_pool::*, game_lib::*, game_map::*, game_obj_lib::*};
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

pub fn explode(
    pos: &Vec2,
    damage: f32,
    explode_range: f32,
    commands: &mut Commands,
    despawn_pool: &mut DespawnPool,
) {

}

pub fn update_obj_pos(
    entity: &Entity,
    new_pos: &Vec2,
    game_obj_lib: &mut GameObjLib,
    game_map: &mut GameMap,
    transform: &mut Transform,
) {
    let Some(obj) = game_obj_lib.get_mut(entity) else {
        error!("Cannot find entity in GameObjLib");
        return;
    };

    obj.pos = new_pos.clone();

    let map_pos = game_map.get_map_pos(&obj.pos);
    game_map.relocate(entity.clone(), &obj.map_pos, &map_pos);
    obj.map_pos = map_pos;

    let screen_pos = game_map.get_screen_pos(&obj.pos);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
}
