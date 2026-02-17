use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

pub fn try_shoot(
    entity: Entity,
    base_speed: f32,
    weapon_comp: &mut WeaponComponent,
    world_info: &WorldInfo,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    new_obj_queue: &mut NewObjQueue,
    time: &Time,
) -> Result<(), MyError> {
    weapon_comp.fire_timer.tick(time.delta());
    if !weapon_comp.fire_timer.is_finished() {
        return Ok(());
    }

    let obj = game_obj_lib.get(&entity).cloned()?;
    let base_velocity = base_speed * obj.direction;

    for i in 0..weapon_comp.fire_points.len() {
        let missile_config = game_lib
            .get_game_obj_config(weapon_comp.missile_config_index)
            .missile_config()?;
        let pos = obj.pos + obj.direction.rotate(weapon_comp.fire_points[i]);
        let relative_direction = obj.direction.rotate(weapon_comp.fire_directions[i]);
        let velocity = relative_direction * missile_config.speed + base_velocity;
        let direction = velocity.normalize();
        let speed = Some(velocity.length());

        if !world_info.check_pos_visible(&pos) {
            continue;
        }

        new_obj_queue.push(NewObj {
            config_index: weapon_comp.missile_config_index,
            pos,
            direction,
            speed,
        });
    }

    Ok(())
}
