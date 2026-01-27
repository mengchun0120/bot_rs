use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;
use core::f32;

pub fn check_collide(
    entity: &Entity,
    pos: &Vec2,
    direction: &Vec2,
    speed: f32,
    obj_config: &GameObjConfig,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
    time_delta: f32,
) -> (bool, f32) {
    let (collide_bounds, time_delta) = check_collide_bounds(
        pos,
        direction,
        speed,
        obj_config.collide_span,
        world_info.world_width(),
        world_info.world_height(),
        time_delta,
    );

    let (collide_obj, time_delta) = check_collide_objs(
        entity,
        pos,
        direction,
        speed,
        obj_config,
        game_map,
        game_obj_lib,
        world_info,
        game_lib,
        despawn_pool,
        time_delta,
    );

    (collide_bounds || collide_obj, time_delta)
}

fn check_collide_bounds(
    pos: &Vec2,
    direction: &Vec2,
    speed: f32,
    collide_span: f32,
    world_width: f32,
    world_height: f32,
    time_delta: f32,
) -> (bool, f32) {
    let velocity = direction * speed;
    if velocity.x == 0.0 && velocity.y == 0.0 {
        return (false, time_delta);
    }

    let tx = if velocity.x > 0.0 {
        (world_width - pos.x - collide_span) / velocity.x
    } else if velocity.x < 0.0 {
        (pos.x - collide_span) / (-velocity.x)
    } else {
        f32::INFINITY
    };

    let ty = if velocity.y > 0.0 {
        (world_height - pos.y - collide_span) / velocity.y
    } else if velocity.y < 0.0 {
        (pos.y - collide_span) / (-velocity.y)
    } else {
        f32::INFINITY
    };

    let t = tx.min(ty);

    if t < time_delta {
        (true, t)
    } else {
        (false, time_delta)
    }
}

fn check_collide_objs(
    entity: &Entity,
    pos: &Vec2,
    direction: &Vec2,
    speed: f32,
    obj_config: &GameObjConfig,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
    time_delta: f32,
) -> (bool, f32) {
    let mut time_delta = time_delta;
    let end_pos = pos + direction * speed * time_delta;
    let region =
        game_map.get_collide_region_bot(pos, &end_pos, obj_config.collide_span, world_info);
    let mut collide_obj = false;
    let func = |e: &Entity| -> bool {
        if e == entity || despawn_pool.contains(e) {
            return true;
        }

        let Some(obj) = game_obj_lib.get(e) else {
            error!("Cannot find entity in GameObjLib");
            return true;
        };
        let obj_config2 = game_lib.get_game_obj_config(obj.config_index);

        if obj_config2.obj_type == GameObjType::Missile
            || obj_config2.obj_type == GameObjType::Explosion
        {
            return true;
        }

        let (collide, new_time_delta) = check_collide_obj(
            pos,
            direction,
            speed,
            obj_config.collide_span,
            &obj.pos,
            obj_config2.collide_span,
            time_delta,
        );

        if collide {
            collide_obj = true;
            time_delta = new_time_delta;
        }

        true
    };

    game_map.run_on_region(&region, func);

    (collide_obj, time_delta)
}

fn check_collide_obj(
    pos1: &Vec2,
    direction: &Vec2,
    speed: f32,
    collide_span1: f32,
    pos2: &Vec2,
    collide_span2: f32,
    time_delta: f32,
) -> (bool, f32) {
    let velocity = direction * speed;
    let span = collide_span1 + collide_span2;
    let d = pos2 - pos1;

    if (d.x >= span && velocity.x <= 0.0)
        || (d.x <= -span && velocity.x >= 0.0)
        || (d.y >= span && velocity.y <= 0.0)
        || (d.y <= -span && velocity.y >= 0.0)
    {
        return (false, time_delta);
    }

    let abs_d = d.abs();
    let abs_vel = velocity.abs();

    if abs_d.x >= span && abs_d.y >= span {
        let tx = (abs_d.x - span) / abs_vel.x;
        let ty = (abs_d.y - span) / abs_vel.y;

        if (tx < ty && ty < time_delta.min((abs_d.x + span) / abs_vel.x))
            || (ty < tx && tx < time_delta.min((abs_d.y + span) / abs_vel.y))
            || (tx == ty && tx < time_delta)
        {
            (true, tx.max(ty))
        } else {
            (false, time_delta)
        }
    } else if abs_d.y >= span {
        let ty = (abs_d.y - span) / abs_vel.y;

        if (abs_vel.x > 0.0 && ty < time_delta.min((abs_d.x + span) / abs_vel.x))
            || (abs_vel.x == 0.0 && ty < time_delta)
        {
            (true, ty)
        } else {
            (false, time_delta)
        }
    } else {
        let tx = (abs_d.x - span) / abs_vel.x;

        if (abs_vel.y > 0.0 && tx < time_delta.min((abs_d.y + span) / abs_vel.y))
            || (abs_vel.y == 0.0 && tx < time_delta)
        {
            (true, tx)
        } else {
            (false, time_delta)
        }
    }
}

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

pub fn check_missile_collide_obj(
    pos1: &Vec2,
    collide_span1: f32,
    pos2: &Vec2,
    collide_span2: f32,
) -> bool {
    let total_span = collide_span1 + collide_span2;
    (pos1.x - pos2.x).abs() < total_span && (pos1.y - pos2.y).abs() < total_span
}
