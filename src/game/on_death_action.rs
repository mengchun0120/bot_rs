use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn on_death(
    entity: Entity,
    hp_query: &mut Query<&mut HPComponent>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        let msg = format!("Cannot find GameObj {}", entity);
        error!(msg);
        return Err(MyError::NotFound(msg));
    };
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let on_death_actions = obj_config.get_on_death_actions()?;

    for action in on_death_actions.iter() {
        match action {
            OnDeathAction::DoDamage => {
                let missile_config = obj_config.missile_config()?;
                do_damage(
                    &obj.pos,
                    missile_config.side,
                    missile_config.damage_range,
                    missile_config.damage,
                    hp_query,
                    game_map,
                    game_obj_lib,
                    game_lib,
                    despawn_pool,
                )?;
            }
            OnDeathAction::PlayFrame(config_name) => {
                play_frame(
                    config_name,
                    &obj.pos,
                    world_info,
                    game_map,
                    game_obj_lib,
                    game_lib,
                    commands,
                )?;
            }
            OnDeathAction::Phaseout(_) => {}
        }
    }

    Ok(())
}

fn do_damage(
    pos: &Vec2,
    side: GameObjSide,
    damage_range: f32,
    damage: f32,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) -> Result<(), MyError> {
    let total_span = damage_range + game_lib.game_config.max_collide_span;
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
        let Ok(obj_config) = game_lib.get_game_obj_config(obj.config_index).bot_config() else {
            continue;
        };

        if obj_config.side != side
            && check_collide_obj(pos, damage_range, &obj.pos, obj_config.collide_span)
            && let Ok(mut hp_comp) = hp_query.get_mut(entity)
        {
            hp_comp.update(-damage);
            if hp_comp.hp() == 0.0 {
                despawn_pool.insert(entity);
            }
        }
    }

    Ok(())
}

fn play_frame(
    config_name: &String,
    pos: &Vec2,
    world_info: &WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let config_index = game_lib.get_game_obj_config_index(config_name)?;
    let direction = Vec2::new(1.0, 0.0);

    create_obj_by_index(
        config_index,
        *pos,
        direction,
        None,
        world_info,
        game_map,
        game_obj_lib,
        game_lib,
        commands,
    )
}
