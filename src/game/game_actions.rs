use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn add_obj_by_config(
    map_obj_config: &GameMapObjConfig,
    game_world: &mut GameWorld,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(&map_obj_config.config_name)?;
    let pos = arr_to_vec2(&map_obj_config.pos);
    let direction = arr_to_vec2(&map_obj_config.direction).normalize();

    add_obj_by_index(
        config_index,
        &pos,
        &direction,
        map_obj_config.speed,
        game_world,
        game_obj_lib,
        game_lib,
        commands,
    )?;

    Ok(())
}

pub fn add_obj_by_index(
    config_index: usize,
    pos: &Vec2,
    direction: &Vec2,
    speed: Option<f32>,
    game_world: &mut GameWorld,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let obj_config = game_lib.get_game_obj_config(config_index);
    let Some((obj, entity)) = GameObj::create(
        config_index,
        pos,
        direction,
        speed,
        game_world,
        game_lib,
        commands,
    )?
    else {
        return Ok(());
    };

    game_world.add(&obj.map_pos, entity);
    game_world.update_max_collide_span(obj_config.collide_span);
    game_obj_lib.insert(entity, obj);

    Ok(())
}

pub fn fire_missiles(
    entity: Entity,
    missile_config_index: usize,
    fire_points: &Vec<Vec2>,
    fire_directions: &Vec<Vec2>,
    base_velocity: &Vec2,
    game_world: &mut GameWorld,
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

        if !game_world.check_pos_visible(&pos) {
            continue;
        }

        add_obj_by_index(
            missile_config_index,
            &pos,
            &direction,
            speed,
            game_world,
            game_obj_lib,
            game_lib,
            commands,
        )?;
    }

    Ok(())
}

pub fn update_obj_pos(
    entity: Entity,
    new_pos: &Vec2,
    game_world: &mut GameWorld,
    game_obj_lib: &mut GameObjLib,
    transform: &mut Transform,
) {
    let Some(obj) = game_obj_lib.get_mut(&entity) else {
        error!("Cannot find entity in GameObjLib");
        return;
    };

    obj.pos = new_pos.clone();

    let map_pos = game_world.get_map_pos(&obj.pos);
    if map_pos != obj.map_pos {
        game_world.relocate(entity.clone(), &obj.map_pos, &map_pos);
        obj.map_pos = map_pos;
    }

    let screen_pos = game_world.get_screen_pos(&obj.pos);
    transform.translation.x = screen_pos.x;
    transform.translation.y = screen_pos.y;
}

pub fn capture_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    captured_missiles: &mut HashSet<Entity>,
    game_world: &GameWorld,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) {
    let total_span = collide_span + game_world.max_collide_span();
    let region = game_world.get_region(
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

    game_world.run_on_region(&region, func);
}

pub fn explode_all(
    missiles: &mut HashSet<Entity>,
    game_obj_lib: &mut GameObjLib,
    game_world: &mut GameWorld,
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
            game_world,
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
    game_world: &mut GameWorld,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(explosion)?;
    let explosion_config = game_lib.get_game_obj_config(config_index);
    let direction = Vec2::new(1.0, 0.0);

    add_obj_by_index(
        config_index,
        pos,
        &direction,
        None,
        game_world,
        game_obj_lib,
        game_lib,
        commands,
    )?;

    if let Some(damage) = explosion_config.damage {
        do_damage(
            pos,
            explosion_config.side,
            damage,
            explosion_config.collide_span,
            game_world,
            game_obj_lib,
            game_lib,
            despawn_pool,
        );
    }

    Ok(())
}

pub fn translate_cursor_pos(
    cursor_pos: Vec2,
    camera: &Camera,
    transform: &GlobalTransform,
    game_world: &GameWorld,
) -> Option<Vec2> {
    let pos = match camera.viewport_to_world_2d(transform, cursor_pos) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to transform cursor position: {}", err);
            return None;
        }
    };

    Some(game_world.viewport_to_world(&pos))
}

fn do_damage(
    pos: &Vec2,
    side: GameObjSide,
    damage: f32,
    span: f32,
    game_world: &GameWorld,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let total_span = span + game_world.max_collide_span();
    let region = game_world.get_region(
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
            && check_collide_obj(pos, span, &obj.pos, obj_config.collide_span)
            && let Some(hp) = obj.hp.as_mut()
        {
            *hp = (*hp - damage).max(0.0);
            if *hp == 0.0 {
                despawn_pool.insert(entity.clone());
            }
        }

        true
    };

    game_world.run_on_region(&region, func);
}
