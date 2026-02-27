use crate::config::*;
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
    speed: f32,
    transform: &mut Transform,
    visibility: &mut Visibility,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
    time: &Time,
) -> Result<MoveResult, MyError> {
    if speed == 0.0 {
        return Ok(MoveResult::NotMoved);
    }

    let obj = game_obj_lib.get(&entity).cloned()?;
    let new_pos = obj.pos + obj.direction * speed * time.delta_secs();
    let collided = check_collide(
        &entity,
        &new_pos,
        obj.collide_span,
        game_lib.game_config.max_collide_span,
        world_info,
        game_map,
        game_obj_lib,
        despawn_pool,
    );

    if !collided {
        update_obj_pos(
            entity,
            new_pos,
            transform,
            world_info,
            game_map,
            game_obj_lib,
        )?;

        if obj.side == GameObjSide::AI {
            update_bot_visibility(entity, &new_pos, visibility, world_info, commands);
        }
    }

    capture_missiles(
        &new_pos,
        obj.collide_span,
        obj.side,
        hp_query,
        game_map,
        game_obj_lib,
        game_lib,
        new_obj_queue,
        despawn_pool,
        commands,
    )?;

    Ok(if collided {
        MoveResult::Collided
    } else {
        MoveResult::Moved(new_pos)
    })
}

pub fn move_missile(
    entity: Entity,
    speed: f32,
    transform: &mut Transform,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
    time: &Time,
) -> Result<MoveResult, MyError> {
    if speed == 0.0 {
        return Ok(MoveResult::NotMoved);
    }

    let obj = game_obj_lib.get(&entity).cloned()?;
    let new_pos = obj.pos + obj.direction * speed * time.delta_secs();

    if !world_info.check_pos_visible(&new_pos) {
        despawn_pool.add(entity, game_obj_lib)?;
        return Ok(MoveResult::NotMoved);
    }

    let collided = check_collide(
        &entity,
        &new_pos,
        obj.collide_span,
        game_lib.game_config.max_collide_span,
        world_info,
        game_map,
        game_obj_lib,
        despawn_pool,
    );
    if collided {
        on_death(
            entity,
            hp_query,
            game_map,
            game_obj_lib,
            game_lib,
            new_obj_queue,
            despawn_pool,
            commands,
        )?;
        Ok(MoveResult::Collided)
    } else {
        update_obj_pos(
            entity,
            new_pos,
            transform,
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
    transform: &mut Transform,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
) -> Result<(), MyError> {
    let obj = game_obj_lib.get_mut(&entity)?;

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
    visibility: &mut Visibility,
    world_info: &WorldInfo,
    commands: &mut Commands,
) {
    if world_info.check_pos_visible(pos) {
        commands.entity(entity).insert(InView);
        *visibility = Visibility::Visible;
    } else {
        commands.entity(entity).remove::<InView>();
        *visibility = Visibility::Hidden;
    }
}

fn capture_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let collided_missiles =
        get_collided_missiles(pos, collide_span, side, game_map, game_obj_lib, game_lib);

    if !collided_missiles.is_empty() {
        for entity in collided_missiles {
            on_death(
                entity,
                hp_query,
                game_map,
                game_obj_lib,
                game_lib,
                new_obj_queue,
                despawn_pool,
                commands,
            )?;
        }
    }

    Ok(())
}

fn get_collided_missiles(
    pos: &Vec2,
    collide_span: f32,
    side: GameObjSide,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
) -> HashSet<Entity> {
    let mut collided_missiles: HashSet<Entity> = HashSet::new();
    let total_span = collide_span + game_lib.game_config.max_collide_span;
    let region = game_map.get_region(
        pos.x - total_span,
        pos.y - total_span,
        pos.x + total_span,
        pos.y + total_span,
    );

    for entity in game_map.map_iter(&region) {
        if let Ok(obj) = game_obj_lib.get(&entity)
            && obj.obj_type == GameObjType::Missile
            && obj.state == GameObjState::Alive
            && obj.side != side
            && check_collide_obj(pos, collide_span, &obj.pos, obj.collide_span)
        {
            collided_missiles.insert(entity);
        }
    }

    collided_missiles
}
