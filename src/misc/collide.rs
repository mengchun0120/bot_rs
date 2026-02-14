use crate::config::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn check_collide(
    entity: &Entity,
    pos: &Vec2,
    collide_span: f32,
    max_collide_span: f32,
    world_info: &WorldInfo,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
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
        max_collide_span,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
    )
}

#[inline]
pub fn get_collide_region(
    pos: &Vec2,
    collide_span: f32,
    max_collide_span: f32,
    game_map: &GameMap,
) -> MapRegion {
    let span = max_collide_span + collide_span;
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
    max_collide_span: f32,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> bool {
    let collide_region = get_collide_region(pos, collide_span, max_collide_span, game_map);

    for e in game_map.map_iter(&collide_region) {
        if *entity == e || despawn_pool.contains(&e) {
            continue;
        }

        let Some(obj2) = game_obj_lib.get(&e) else {
            error!("Cannot find GameObj {} in GameObjLib", e);
            continue;
        };
        let named_config = &game_lib.get_game_obj_config(obj2.config_index);

        let collide_span2 = match &named_config.config {
            GameObjConfig::Bot(config) => config.collide_span,
            GameObjConfig::Tile(config) => config.collide_span,
            _ => {
                continue;
            }
        };

        if check_collide_obj(&pos, collide_span, &obj2.pos, collide_span2) {
            return true;
        }
    }

    false
}
