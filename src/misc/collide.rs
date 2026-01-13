use crate::config::game_obj_config::*;
use crate::game_utils::{despawn_pool::*, game_lib::*, game_map::*, game_obj_lib::*};
use bevy::prelude::*;

pub fn get_bot_pos_after_collide_bounds(
    pos: &Vec2,
    collide_span: f32,
    direction: &Vec2,
    width: f32,
    height: f32,
) -> (bool, Vec2) {
    let left = pos.x - collide_span;
    let right = pos.x + collide_span;
    let dx = if left < 0.0 {
        -left
    } else if right > width {
        width - right
    } else {
        0.0
    };

    let bottom = pos.y - collide_span;
    let top = pos.y + collide_span;
    let dy = if bottom < 0.0 {
        -bottom
    } else if top > height {
        height - top
    } else {
        0.0
    };

    let mut corrected_pos = pos.clone();
    let min_x = collide_span;
    let max_x = width - collide_span;
    let min_y = collide_span;
    let max_y = height - collide_span;

    let collide = if dx == 0.0 && dy == 0.0 {
        false
    } else {
        if dx.signum() * direction.x.signum() < 0.0 && dy.signum() * direction.y.signum() < 0.0 {
            if (dx * direction.y).abs() < (dy * direction.x).abs() {
                corrected_pos.x = corrected_pos.x.clamp(min_x, max_x);
                corrected_pos.y += dy.signum() * (dx * direction.y / direction.x).abs();
                corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
            } else {
                corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
                corrected_pos.x += dx.signum() * (dy * direction.x / direction.y).abs();
                corrected_pos.x = corrected_pos.x.clamp(min_y, max_y);
            }
        } else {
            corrected_pos.x = corrected_pos.x.clamp(min_x, max_x);
            corrected_pos.y = corrected_pos.y.clamp(min_y, max_y);
        }
        true
    };

    (collide, corrected_pos)
}

pub fn get_bot_pos_after_collide_obj(
    pos1: &Vec2,
    collide_span1: f32,
    direction: &Vec2,
    pos2: &Vec2,
    collide_span2: f32,
) -> (bool, Vec2) {
    let total_span = collide_span1 + collide_span2;
    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();
    let mut corrected_pos = pos1.clone();

    if dx >= total_span || dy >= total_span {
        return (false, corrected_pos);
    }

    let cx = total_span - dx;
    let cy = total_span - dy;

    if cx * direction.y.abs() < cy * direction.x.abs() {
        corrected_pos.x = if direction.x > 0.0 {
            pos2.x - total_span
        } else {
            pos2.x + total_span
        };
        if direction.y != 0.0 {
            corrected_pos.y -= direction.y.signum() * cx * direction.y.abs() / direction.x;
        }
    } else {
        corrected_pos.y = if direction.y > 0.0 {
            pos2.y - total_span
        } else {
            pos2.y + total_span
        };
        if direction.x != 0.0 {
            corrected_pos.x -= direction.x.signum() * cy * direction.x.abs() / direction.y;
        }
    }

    (true, corrected_pos)
}

pub fn check_missile_collide(
    new_pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> bool {
    check_missile_collide_bounds(new_pos, collide_span, game_map.width, game_map.height)
        || check_missile_collide_objs(
            new_pos,
            collide_span,
            side,
            game_map,
            game_obj_lib,
            game_lib,
            despawn_pool,
        )
}

pub fn check_missile_collide_objs(
    new_pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> bool {
    let region = game_map.get_region(
        new_pos.y - collide_span,
        new_pos.y + collide_span,
        new_pos.x - collide_span,
        new_pos.x + collide_span,
    );
    let mut collide = false;
    let func = |entity: &Entity| -> bool {
        if despawn_pool.contains(entity) {
            return true;
        }

        let Some(obj) = game_obj_lib.get(entity) else {
            error!("Cannot find entity in GameObjLib");
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if (obj_config.obj_type != GameObjType::Bot && obj_config.obj_type != GameObjType::Tile)
            || (obj_config.obj_type == GameObjType::Bot && obj_config.side == side)
            || obj_config.collide_span == 0.0
        {
            return true;
        }

        if check_missile_collide_bounds(new_pos, collide_span, game_map.width, game_map.height)
            || check_missile_collide_obj(new_pos, collide_span, &obj.pos, obj_config.collide_span)
        {
            collide = true;
            return false;
        }

        true
    };

    game_map.run_on_region(&region, func);

    collide
}

fn check_missile_collide_bounds(pos: &Vec2, collide_span: f32, width: f32, height: f32) -> bool {
    pos.x - collide_span < 0.0
        || pos.x + collide_span > width
        || pos.y - collide_span < 0.0
        || pos.y + collide_span > height
}

fn check_missile_collide_obj(
    pos1: &Vec2,
    collide_span1: f32,
    pos2: &Vec2,
    collide_span2: f32,
) -> bool {
    let total_span = collide_span1 + collide_span2;
    (pos1.x - pos2.x).abs() < total_span && (pos1.y - pos2.y).abs() < total_span
}
