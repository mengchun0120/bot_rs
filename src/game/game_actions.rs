use crate::config::*;
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

pub fn capture_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    captured_missiles: &mut HashSet<Entity>,
    game_obj_lib: &GameObjLib,
    game_map: &GameMap,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) {
    let total_span = collide_span + game_map.max_collide_span;
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
            && check_missile_collide_obj(pos, collide_span, &obj.pos, obj_config.collide_span)
        {
            captured_missiles.insert(entity.clone());
        }

        true
    };

    game_map.run_on_region(&region, func);
}

pub fn explode_all(
    missiles: &mut HashSet<Entity>,
    game_obj_lib: &mut GameObjLib,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    for entity in missiles.iter() {
        let Some((config_index, pos)) = game_obj_lib
            .get(entity)
            .map(|obj| (obj.config_index, obj.pos))
        else {
            error!("Cannot find entity in GameObjLib");
            continue;
        };
        let Some(explosion) = game_lib
            .get_game_obj_config(config_index)
            .explosion
            .as_ref()
        else {
            continue;
        };

        let _ = explode(
            explosion,
            &pos,
            game_obj_lib,
            game_map,
            game_lib,
            despawn_pool,
            commands,
        );

        despawn_pool.insert(entity.clone());
    }

    missiles.clear();
}

pub fn explode(
    explosion: &String,
    pos: &Vec2,
    game_obj_lib: &mut GameObjLib,
    game_map: &mut GameMap,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(explosion)?;
    let explosion_config = game_lib.get_game_obj_config(config_index);
    let direction = Vec2::new(1.0, 0.0);

    game_map.add_obj_by_index(
        config_index,
        pos,
        &direction,
        game_lib,
        game_obj_lib,
        commands,
    )?;

    if let Some(damage) = explosion_config.damage {
        do_damage(
            pos,
            explosion_config.side,
            damage,
            explosion_config.collide_span,
            game_map,
            game_obj_lib,
            game_lib,
            despawn_pool,
        );
    }

    Ok(())
}

pub fn get_cursor_pos(
    window: &Window,
    camera: &Camera,
    transform: &GlobalTransform,
    game_map: &GameMap,
) -> Option<Vec2> {
    let Some(cursor_pos) = window.cursor_position() else {
        return None;
    };
    let pos = match camera.viewport_to_world_2d(transform, cursor_pos) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to transform cursor position: {}", err);
            return None;
        }
    };

    Some(game_map.viewport_to_world(&pos))
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

fn do_damage(
    pos: &Vec2,
    side: GameObjSide,
    damage: f32,
    span: f32,
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let total_span = span + game_map.max_collide_span;
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

        let Some(obj) = game_obj_lib.get_mut(entity) else {
            error!("Cannot find entity in GameObjLib");
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.obj_type == GameObjType::Bot
            && obj_config.side != side
            && check_missile_collide_obj(pos, span, &obj.pos, obj_config.collide_span)
            && let Some(hp) = obj.hp.as_mut()
        {
            *hp = (*hp - damage).max(0.0);
            if *hp == 0.0 {
                despawn_pool.insert(entity.clone());
            }
        }

        true
    };

    game_map.run_on_region(&region, func);
}
