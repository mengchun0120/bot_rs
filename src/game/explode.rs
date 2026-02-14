use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn explode_all(
    missiles: &mut HashSet<Entity>,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
) {
    for entity in missiles.iter() {
        let Some(obj) = game_obj_lib.get(entity) else {
            error!("Cannot find GameObj");
            continue;
        };
        let Ok(config) = game_lib
            .get_game_obj_config(obj.config_index)
            .missile_config()
        else {
            return;
        };

        let _ = explode(
            &config.explosion,
            obj.pos,
            hp_query,
            game_map,
            game_obj_lib,
            game_lib,
            new_obj_queue,
            despawn_pool,
        );

        despawn_pool.insert(entity.clone());
    }

    missiles.clear();
}

pub fn explode(
    explosion: &String,
    pos: Vec2,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(explosion)?;
    let explosion_config = game_lib
        .get_game_obj_config(config_index)
        .explosion_config()?;
    let direction = Vec2::new(1.0, 0.0);

    do_damage(
        pos,
        explosion_config.side,
        explosion_config.damage,
        explosion_config.collide_span,
        hp_query,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
    );

    new_obj_queue.push(NewObj {
        config_index,
        pos,
        direction,
        speed: None,
    });

    Ok(())
}

fn do_damage(
    pos: Vec2,
    side: GameObjSide,
    damage: f32,
    span: f32,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let total_span = span + game_lib.game_config.max_collide_span;
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
            error!("Cannot find GameObj {} in GameObjLib", entity);
            continue;
        };
        let Ok(config) = game_lib.get_game_obj_config(obj.config_index).bot_config() else {
            continue;
        };

        if config.side != side
            && check_collide_obj(&pos, span, &obj.pos, config.collide_span)
            && let Ok(mut hp_comp) = hp_query.get_mut(entity)
        {
            hp_comp.update(-damage);
            if hp_comp.hp() == 0.0 {
                despawn_pool.insert(entity);
            }
        }
    }
}
