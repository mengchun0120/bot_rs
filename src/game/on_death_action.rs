use crate::config::{GameObjSide, OnDeathAction, SpawnMissileConfig};
use crate::game::{
    GameObjState, GameObjType, Phaseout,
    components::{AiBotComponent, PlayoutComponent},
};
use crate::game_utils::{GameLib, GameMap, GameObjLib, NewObj, NewObjQueue};
use crate::misc::{MyError, check_collide_obj};
use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

pub fn on_death(
    entity: Entity,
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    commands: &mut Commands,
) -> Result<(), MyError> {
    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        let msg = "Failed to find obj in GameObjLib".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let on_death_actions = obj_config.get_on_death_actions()?;

    for action in on_death_actions.iter() {
        match action {
            OnDeathAction::DoDamage(damage_config) => {
                on_do_damage(
                    &obj.pos,
                    obj.side,
                    damage_config.damage_range,
                    damage_config.damage,
                    game_map,
                    game_obj_lib,
                    game_lib,
                    new_obj_queue,
                    commands,
                )?;
            }
            OnDeathAction::PlayFrame(config_name) => {
                on_play_frame(config_name, &obj.pos, game_lib, new_obj_queue)?;
            }
            OnDeathAction::Phaseout(duration) => {
                on_phaseout(entity, *duration, game_obj_lib, commands)?;
            }
            OnDeathAction::SpawnMissile(spawn_missile_config) => {
                on_spawn_missile(&obj.pos, spawn_missile_config, game_lib, new_obj_queue)?;
            }
            OnDeathAction::DropGoodie(prob) => {
                on_drop_goodie(&obj.pos, *prob, game_lib, new_obj_queue)?;
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
    game_map: &GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
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
        if let Ok(obj) = game_obj_lib.get_mut(&entity)
            && obj.state == GameObjState::Alive
            && obj.side != side
            && obj.obj_type == GameObjType::Bot
            && check_collide_obj(pos, damage_range, &obj.pos, obj.collide_span)
        {
            if let Some(hp) = obj.hp {
                let new_hp = (hp - damage).max(0.0);
                obj.hp = Some(new_hp);
                if new_hp == 0.0 {
                    on_death(
                        entity,
                        game_map,
                        game_obj_lib,
                        game_lib,
                        new_obj_queue,
                        commands,
                    )?;
                }
            } else {
                error!("Bot's hp is none");
                continue;
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
    commands: &mut Commands,
) -> Result<(), MyError> {
    let obj = game_obj_lib.get_mut(&entity)?;

    if obj.state != GameObjState::Alive {
        let msg = format!("Failed to phaseout: GameObj {} is not alive", entity);
        error!(msg);
        return Err(MyError::Other(msg));
    }

    let phaseout = Phaseout::new(duration);
    let mut cmd = commands.entity(entity);

    obj.state = GameObjState::Phaseout;
    if obj.is_ai_bot() {
        cmd.remove::<AiBotComponent>();
    }
    cmd.insert(PlayoutComponent::new(phaseout));

    Ok(())
}

fn on_spawn_missile(
    pos: &Vec2,
    spawn_missile_config: &SpawnMissileConfig,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
) -> Result<(), MyError> {
    let mut angle: f32 = 0.0;
    let delta_angle = 2.0 * std::f32::consts::PI / spawn_missile_config.count as f32;
    let config_index = game_lib.get_game_obj_config_index(&spawn_missile_config.missile)?;

    for _ in 0..spawn_missile_config.count {
        let new_obj = NewObj {
            config_index,
            pos: pos.clone(),
            direction: Vec2::new(angle.cos(), angle.sin()),
            speed: None,
        };
        new_obj_queue.push(new_obj);
        angle += delta_angle;
    }

    Ok(())
}

fn on_drop_goodie(
    pos: &Vec2,
    prob: f32,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
) -> Result<(), MyError> {
    let mut rng = rand::rng();

    if rng.random_range(0.0..1.0) > prob {
        return Ok(());
    }

    if let Some(config_index) = game_lib.goodies().choose(&mut rng) {
        let new_obj = NewObj {
            config_index: *config_index,
            pos: pos.clone(),
            direction: Vec2::new(1.0, 0.0),
            speed: None,
        };
        new_obj_queue.push(new_obj);
    }

    Ok(())
}
