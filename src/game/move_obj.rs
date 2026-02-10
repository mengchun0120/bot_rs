use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveResult {
    Moved(Vec2),
    Collided,
    NotMoved,
}

pub fn move_bot(
    entity: Entity,
    move_comp_query: &mut Query<&mut MoveComponent>,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
    time: &Time,
) -> Result<MoveResult, MyError> {
    let Ok(mut move_comp) = move_comp_query.get_mut(entity) else {
        let msg = "Cannot find MoveComponent".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    if move_comp.speed == 0.0 {
        return Ok(MoveResult::NotMoved);
    }

    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        let msg = "Cannot find GameObj".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let new_pos = obj.pos + obj.direction * move_comp.speed * time.delta_secs();
    let collided = check_collide(
        &entity,
        &new_pos,
        obj_config.collide_span,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
    );

    if !collided {
        update_obj_pos(
            entity,
            new_pos,
            transform_query,
            world_info,
            game_map,
            game_obj_lib,
        )?;

        if obj_config.side == GameObjSide::AI {
            update_bot_visibility(entity, &new_pos, visibility_query, world_info, commands)?;
        }
    } else if obj_config.side == GameObjSide::AI {
        move_comp.speed = 0.0;
    }

    capture_missiles(
        &new_pos,
        obj_config.collide_span,
        obj_config.side,
        hp_query,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        new_obj_queue,
        despawn_pool,
    );

    Ok(if collided {
        MoveResult::Collided
    } else {
        MoveResult::Moved(new_pos)
    })
}

pub fn stop_bot(
    entity: Entity,
    move_comp_query: &mut Query<&mut MoveComponent>,
) -> Result<(), MyError> {
    let Ok(mut move_comp) = move_comp_query.get_mut(entity) else {
        let msg = "Cannot find MoveComponent".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    move_comp.speed = 0.0;

    Ok(())
}

pub fn move_missile(
    entity: Entity,
    move_comp_query: &Query<&mut MoveComponent>,
    transform_query: &mut Query<&mut Transform>,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    time: &Time,
) -> Result<MoveResult, MyError> {
    let Ok(move_comp) = move_comp_query.get(entity) else {
        let msg = "Cannot find MoveComponent".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    if move_comp.speed == 0.0 {
        return Ok(MoveResult::NotMoved);
    }

    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        let msg = "Cannot find GameObj".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let new_pos = obj.pos + obj.direction * move_comp.speed * time.delta_secs();

    if !world_info.check_pos_visible(&new_pos) {
        despawn_pool.insert(entity);
        return Ok(MoveResult::NotMoved);
    }

    let collided = check_collide(
        &entity,
        &new_pos,
        obj_config.collide_span,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
    );
    if collided {
        if let Some(explosion) = obj_config.explosion.as_ref() {
            explode(
                explosion,
                new_pos,
                hp_query,
                world_info,
                game_map,
                game_obj_lib,
                game_lib,
                new_obj_queue,
                despawn_pool,
            )?;
        }
        despawn_pool.insert(entity);
        Ok(MoveResult::Collided)
    } else {
        update_obj_pos(
            entity,
            new_pos,
            transform_query,
            world_info,
            game_map,
            game_obj_lib,
        )?;
        Ok(MoveResult::Moved(new_pos))
    }
}

fn update_obj_pos(
    entity: Entity,
    new_pos: Vec2,
    transform_query: &mut Query<&mut Transform>,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
) -> Result<(), MyError> {
    let Some(obj) = game_obj_lib.get_mut(&entity) else {
        let msg = "Cannot find GameObj".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };
    let Ok(mut transform) = transform_query.get_mut(entity) else {
        let msg = "Cannot find Transform".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
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

    Ok(())
}

fn update_bot_visibility(
    entity: Entity,
    pos: &Vec2,
    visibility_query: &mut Query<&mut Visibility>,
    world_info: &WorldInfo,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let Ok(mut visibility) = visibility_query.get_mut(entity) else {
        let msg = "Cannot find visibility".to_string();
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    if world_info.check_pos_visible(pos) {
        commands.entity(entity).insert(InView);
        *visibility = Visibility::Visible;
    } else {
        commands.entity(entity).remove::<InView>();
        *visibility = Visibility::Hidden;
    }

    Ok(())
}

fn capture_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
) {
    let mut collided_missiles = get_collided_missiles(
        pos,
        collide_span,
        side,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
    );

    if !collided_missiles.is_empty() {
        explode_all(
            &mut collided_missiles,
            hp_query,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            new_obj_queue,
            despawn_pool,
        );
    }
}

fn get_collided_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    world_info: &WorldInfo,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
) -> HashSet<Entity> {
    let mut collided_missiles: HashSet<Entity> = HashSet::new();
    let total_span = collide_span + world_info.max_collide_span();
    let region = game_map.get_region(
        pos.x - total_span,
        pos.y - total_span,
        pos.x + total_span,
        pos.y + total_span,
    );

    for entity in game_map.map_iter(&region) {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get(&entity) else {
            error!("Cannot find GameObj");
            continue;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.obj_type == GameObjType::Missile
            && obj_config.side != side
            && check_collide_obj(pos, collide_span, &obj.pos, obj_config.collide_span)
        {
            collided_missiles.insert(entity);
        }
    }

    collided_missiles
}
