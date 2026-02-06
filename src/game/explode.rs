use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn explode_all(
    missiles: &mut HashSet<Entity>,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    obj_query: &Query<&mut GameObj>,
    hp_query: &mut Query<&mut HPComponent>,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    for entity in missiles.iter() {
        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj");
            continue;
        };
        let Some(explosion) = game_lib
            .get_game_obj_config(obj.config_index)
            .explosion
            .as_ref()
        else {
            continue;
        };

        let _ = explode(
            explosion,
            obj.pos,
            game_map,
            world_info,
            obj_query,
            hp_query,
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
    pos: Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    obj_query: &Query<&mut GameObj>,
    hp_query: &mut Query<&mut HPComponent>,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(explosion)?;
    let explosion_config = game_lib.get_game_obj_config(config_index);
    let direction = Vec2::new(1.0, 0.0);

    create_obj_by_index(
        config_index,
        pos,
        direction,
        None,
        world_info,
        game_map,
        game_lib,
        commands,
    )?;

    if let Some(damage) = explosion_config.damage {
        do_damage(
            pos,
            explosion_config.side,
            damage,
            explosion_config.collide_span,
            game_map,
            world_info,
            obj_query,
            hp_query,
            game_lib,
            despawn_pool,
        );
    }

    Ok(())
}

fn do_damage(
    pos: Vec2,
    side: GameObjSide,
    damage: f32,
    span: f32,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_query: &Query<&mut GameObj>,
    hp_query: &mut Query<&mut HPComponent>,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let total_span = span + world_info.max_collide_span();
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

        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.obj_type == GameObjType::Bot
            && obj_config.side != side
            && check_collide_obj(&pos, span, &obj.pos, obj_config.collide_span)
            && let Ok(mut hp_comp) = hp_query.get_mut(*entity)
        {
            hp_comp.update(-damage);
            if hp_comp.hp() == 0.0 {
                despawn_pool.insert(*entity);
            }
        }

        true
    };

    game_map.run_on_region(&region, func);
}
