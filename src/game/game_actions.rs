use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn fire_missiles(
    entity: Entity,
    missile_config_index: usize,
    fire_points: &Vec<Vec2>,
    fire_directions: &Vec<Vec2>,
    base_velocity: &Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        error!("Cannot find entity in GameObjLib");
        return Err(MyError::NotFound("entity".into()));
    };

    for i in 0..fire_points.len() {
        let missile_config = game_lib.get_game_obj_config(missile_config_index);
        let pos = obj.pos + obj.direction.rotate(fire_points[i]);
        let relative_direction = obj.direction.rotate(fire_directions[i]);
        let velocity = relative_direction * missile_config.speed + base_velocity;
        let direction = velocity.normalize();
        let speed = Some(velocity.length());

        if !world_info.check_pos_visible(&pos) {
            continue;
        }

        create_obj_by_index(
            missile_config_index,
            pos,
            direction,
            speed,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        )?;
    }

    Ok(())
}

pub fn update_obj_pos(
    entity: Entity,
    new_pos: Vec2,
    game_map: &mut GameMap,
    world_info: &WorldInfo,
    game_obj_lib: &mut GameObjLib,
    transform: &mut Transform,
) {
    let Some(obj) = game_obj_lib.get_mut(&entity) else {
        error!("Cannot find entity in GameObjLib");
        return;
    };

    obj.pos = new_pos;

    let map_pos = game_map.get_map_pos(&obj.pos);
    if map_pos != obj.map_pos {
        game_map.relocate(entity.clone(), &obj.map_pos, &map_pos);
        obj.map_pos = map_pos;
    }

    let screen_pos = world_info.get_screen_pos(&obj.pos);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
}

pub fn capture_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    captured_missiles: &mut HashSet<Entity>,
    game_map: &GameMap,
    world_info: &WorldInfo,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) {
    let total_span = collide_span + world_info.max_collide_span();
    let region = game_map.get_region(
        pos.x - total_span,
        pos.y - total_span,
        pos.x + total_span,
        pos.y + total_span,
    );
    let func = |entity: &Entity| -> bool {
        if despawn_pool.contains(entity) {
            return true;
        }

        let Some(obj) = game_obj_lib.get(entity) else {
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.obj_type == GameObjType::Missile
            && obj_config.side != side
            && check_collide_obj(pos, collide_span, &obj.pos, obj_config.collide_span)
        {
            captured_missiles.insert(entity.clone());
        }

        true
    };

    game_map.run_on_region(&region, func);
}

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
