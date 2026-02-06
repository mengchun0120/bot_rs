use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn fire_missiles(
    pos: Vec2,
    direction: Vec2,
    missile_config_index: usize,
    fire_points: &Vec<Vec2>,
    fire_directions: &Vec<Vec2>,
    base_velocity: &Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    for i in 0..fire_points.len() {
        let missile_config = game_lib.get_game_obj_config(missile_config_index);
        let pos = pos + direction.rotate(fire_points[i]);
        let relative_direction = direction.rotate(fire_directions[i]);
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
            game_lib,
            commands,
        )?;
    }

    Ok(())
}

pub fn update_obj_pos<T, U>(
    entity: Entity,
    new_pos: Vec2,
    game_map: &mut GameMap,
    world_info: &WorldInfo,
    mut obj_mapper: T,
    mut transform_mapper: U,
) where
    T: MutMapper<Entity, GameObj>,
    U: MutMapper<Entity, Transform>,
{
    let Some(obj) = obj_mapper.get(entity) else {
        error!("Cannot find GameObj");
        return;
    };
    let Some(transform) = transform_mapper.get(entity) else {
        error!("Cannot find Transform");
        return;
    };

    obj.pos = new_pos;

    let map_pos = game_map.get_map_pos(&obj.pos);
    if map_pos != obj.map_pos {
        game_map.relocate(entity, &obj.map_pos, &map_pos);
        obj.map_pos = map_pos;
    }

    let screen_pos = world_info.get_screen_pos(&obj.pos);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
}

pub fn capture_missiles<T>(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    captured_missiles: &mut HashSet<Entity>,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_mapper: &T,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) where
    T: Mapper<Entity, GameObj>,
{
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

        let Some(obj) = obj_mapper.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.obj_type == GameObjType::Missile
            && obj_config.side != side
            && check_collide_obj(pos, collide_span, &obj.pos, obj_config.collide_span)
        {
            captured_missiles.insert(*entity);
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
