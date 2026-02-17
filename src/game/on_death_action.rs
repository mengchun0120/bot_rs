use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn on_death(
    entity: Entity,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let obj = game_obj_lib.get(&entity).cloned()?;
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let on_death_actions = obj_config.get_on_death_actions()?;

    for action in on_death_actions.iter() {
        match action {
            OnDeathAction::DoDamage => {
                let missile_config = obj_config.missile_config()?;
                on_do_damage(
                    &obj.pos,
                    missile_config.side,
                    missile_config.damage_range,
                    missile_config.damage,
                    hp_query,
                    game_map,
                    game_obj_lib,
                    game_lib,
                    new_obj_queue,
                    despawn_pool,
                    commands,
                )?;
            }
            OnDeathAction::PlayFrame(config_name) => {
                on_play_frame(config_name, &obj.pos, game_lib, new_obj_queue)?;
            }
            OnDeathAction::Phaseout(duration) => {
                on_phaseout(entity, *duration, game_obj_lib, game_lib, commands)?;
            }
        }
    }

    Ok(())
}

fn on_do_damage(
    pos: &Vec2,
    side: GameObjSide,
    damage_range: f32,
    damage: f32,
    hp_query: &mut Query<&mut HPComponent>,
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
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

        let Ok(obj) = game_obj_lib.get(&entity) else {
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
    }

    Ok(())
}

fn on_play_frame(
    config_name: &String,
    pos: &Vec2,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
) -> Result<(), MyError> {
    let new_obj = NewObj {
        config_index: game_lib.get_game_obj_config_index(config_name)?,
        pos: pos.clone(),
        direction: Vec2::new(1.0, 0.0),
        speed: None,
    };
    new_obj_queue.push(new_obj);
    Ok(())
}

fn on_phaseout(
    entity: Entity,
    duration: f32,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let obj = game_obj_lib.get_mut(&entity)?;
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let phaseout = Phaseout::new(duration);
    let mut cmd = commands.entity(entity);

    obj.is_phaseout = true;
    if obj_config.is_ai_bot() {
        cmd.remove::<AIBotComponent>();
    }
    cmd.insert(PlayoutComponent::new(phaseout));

    Ok(())
}
