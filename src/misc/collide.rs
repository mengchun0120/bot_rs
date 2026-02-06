use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn check_collide(
    entity: &Entity,
    pos: &Vec2,
    collide_span: f32,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_query: &Query<&mut GameObj>,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> bool {
    check_collide_bounds(
        pos,
        collide_span,
        world_info.world_width(),
        world_info.world_height(),
    ) || check_collide_objs(
        entity,
        pos,
        collide_span,
        game_map,
        world_info,
        obj_query,
        game_lib,
        despawn_pool,
    )
}

#[inline]
pub fn get_collide_region(
    pos: &Vec2,
    collide_span: f32,
    game_map: &GameMap,
    world_info: &WorldInfo,
) -> MapRegion {
    let span = world_info.max_collide_span() + collide_span;
    game_map.get_region(pos.x - span, pos.y - span, pos.x + span, pos.y + span)
}

#[inline]
pub fn check_collide_obj(pos1: &Vec2, collide_span1: f32, pos2: &Vec2, collide_span2: f32) -> bool {
    let total_span = collide_span1 + collide_span2;
    let d = (pos2 - pos1).abs();
    d.x < total_span && d.y < total_span
}

#[inline]
fn check_collide_bounds(new_pos: &Vec2, collide_span: f32, width: f32, height: f32) -> bool {
    new_pos.x - collide_span < 0.0
        || new_pos.x + collide_span > width
        || new_pos.y - collide_span < 0.0
        || new_pos.y + collide_span > height
}

fn check_collide_objs(
    entity: &Entity,
    pos: &Vec2,
    collide_span: f32,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_query: &Query<&mut GameObj>,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> bool {
    let mut collide = false;
    let collide_region = get_collide_region(pos, collide_span, game_map, world_info);

    let func = |e: &Entity| -> bool {
        if entity == e || despawn_pool.contains(e) {
            return true;
        }

        let Ok(obj2) = obj_query.get(*e) else {
            error!("Cannot find GameObj");
            return true;
        };
        let obj_config2 = game_lib.get_game_obj_config(obj2.config_index);

        if (obj_config2.obj_type != GameObjType::Bot && obj_config2.obj_type != GameObjType::Tile)
            || obj_config2.collide_span == 0.0
        {
            return true;
        }

        if check_collide_obj(&pos, collide_span, &obj2.pos, obj_config2.collide_span) {
            collide = true;
            return false;
        }

        true
    };

    game_map.run_on_region(&collide_region, func);

    collide
}
