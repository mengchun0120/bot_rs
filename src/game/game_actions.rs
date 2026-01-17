use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::{collide::*, my_error::*};
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

pub fn get_bot_new_pos(
    entity: &Entity,
    obj: &GameObj,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    time: &Time,
) -> (bool, Vec2) {
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let new_pos = obj.pos + obj.direction * obj_config.speed * time.delta_secs();

    let (collide_bounds, new_pos) = get_bot_pos_after_collide_bounds(
        &new_pos,
        obj_config.collide_span,
        &obj.direction,
        game_map.width,
        game_map.height,
    );

    let (collide_obj, new_pos) = get_bot_pos_after_collide_objs(
        entity,
        obj,
        &new_pos,
        game_map,
        obj_config,
        game_obj_lib,
        game_lib,
    );

    (collide_bounds || collide_obj, new_pos)
}

pub fn update_obj_pos(
    entity: Entity,
    new_pos: &Vec2,
    game_obj_lib: &mut GameObjLib,
    game_map: &mut GameMap,
    transform: &mut Transform,
) {
    let Some(obj) = game_obj_lib.get_mut(&entity) else {
        error!("Cannot find entity in GameObjLib");
        return;
    };

    obj.pos = new_pos.clone();

    let map_pos = game_map.get_map_pos(&obj.pos);
    if map_pos != obj.map_pos {
        game_map.relocate(entity.clone(), &obj.map_pos, &map_pos);
        obj.map_pos = map_pos;
    }

    let screen_pos = game_map.get_screen_pos(&obj.pos);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
}

fn get_bot_pos_after_collide_objs(
    entity: &Entity,
    obj: &GameObj,
    new_pos: &Vec2,
    game_map: &GameMap,
    obj_config: &GameObjConfig,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
) -> (bool, Vec2) {
    let mut collide = false;
    let collide_region =
        game_map.get_collide_region_bot(&obj.pos, new_pos, obj_config.collide_span);
    let mut pos = new_pos.clone();
    let func = |e: &Entity| -> bool {
        if entity == e {
            return true;
        }

        let Some(obj2) = game_obj_lib.get(e) else {
            error!("Cannot find entity in GameObjLib");
            return true;
        };
        let obj_config2 = game_lib.get_game_obj_config(obj2.config_index);

        if (obj_config2.obj_type != GameObjType::Bot && obj_config2.obj_type != GameObjType::Tile)
            || obj_config2.collide_span == 0.0
        {
            return true;
        }

        let (collide_obj, corrected_pos) = get_bot_pos_after_collide_obj(
            &pos,
            obj_config.collide_span,
            &obj.direction,
            &obj2.pos,
            obj_config2.collide_span,
        );

        if collide_obj {
            collide = true;
        }

        pos = corrected_pos;

        true
    };

    game_map.run_on_region(&collide_region, func);

    (collide, pos)
}
