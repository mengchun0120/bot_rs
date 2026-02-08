use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn try_shoot(
    entity: Entity,
    move_comp_query: &Query<&mut MoveComponent>,
    weapon_comp_query: &mut Query<&mut WeaponComponent>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
    time: &Time,
) -> Result<(), MyError> {
    let Ok(mut weapon_comp) = weapon_comp_query.get_mut(entity) else {
        let msg = format!("Cannot find WeaponComponent {}", entity);
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    weapon_comp.fire_timer.tick(time.delta());
    if !weapon_comp.fire_timer.is_finished() {
        return Ok(());
    }

    let Ok(move_comp) = move_comp_query.get(entity) else {
        let msg = format!("Cannot find MoveComponent {}", entity);
        error!(msg);
        return Err(MyError::NotFound(msg));
    };
    let Some(obj) = game_obj_lib.get(&entity).cloned() else {
        let msg = format!("Cannot find GameObj {}", entity);
        error!(msg);
        return Err(MyError::NotFound(msg));
    };

    let base_velocity = move_comp.speed * obj.direction;

    for i in 0..weapon_comp.fire_points.len() {
        let missile_config = game_lib.get_game_obj_config(weapon_comp.missile_config_index);
        let pos = obj.pos + obj.direction.rotate(weapon_comp.fire_points[i]);
        let relative_direction = obj.direction.rotate(weapon_comp.fire_directions[i]);
        let velocity = relative_direction * missile_config.speed + base_velocity;
        let direction = velocity.normalize();
        let speed = Some(velocity.length());

        if !world_info.check_pos_visible(&pos) {
            continue;
        }

        create_obj_by_index(
            weapon_comp.missile_config_index,
            pos,
            direction,
            speed,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            commands,
        )?;
    }

    Ok(())
}
