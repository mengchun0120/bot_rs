use crate::config::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponComponent {
    pub fire_timer: Timer,
    pub fire_points: Vec<Vec2>,
    pub fire_directions: Vec<Vec2>,
    pub missile_indices: Vec<usize>,
}

impl WeaponComponent {
    pub fn new(weapon_config: &WeaponConfig, game_lib: &GameLib) -> Result<Self, MyError> {
        let fire_timer = Timer::from_seconds(weapon_config.fire_duration, TimerMode::Repeating);
        let fire_points = Self::get_fire_points(weapon_config, game_lib)?;
        let fire_directions = Self::get_fire_directions(weapon_config);
        let missile_indices = Self::get_missile_indices(weapon_config, game_lib)?;

        Ok(Self {
            fire_timer,
            fire_points,
            fire_directions,
            missile_indices,
        })
    }

    fn get_fire_points(
        weapon_config: &WeaponConfig,
        game_lib: &GameLib,
    ) -> Result<Vec<Vec2>, MyError> {
        let mut fire_points = Vec::new();

        for gun_comp_config in weapon_config.gun_components.iter() {
            let gun_config = game_lib.get_gun_config(&gun_comp_config.config_name)?;
            let gun_pos = arr_to_vec2(&gun_comp_config.pos);
            let gun_direction = arr_to_vec2(&gun_comp_config.direction);
            let local_fire_point = arr_to_vec2(&gun_config.fire_point);
            let fire_point = gun_pos + gun_direction.rotate(local_fire_point);
            fire_points.push(fire_point);
        }

        Ok(fire_points)
    }

    fn get_fire_directions(weapon_config: &WeaponConfig) -> Vec<Vec2> {
        let mut fire_directions = Vec::new();

        for gun_comp_config in weapon_config.gun_components.iter() {
            let fire_direction = arr_to_vec2(&gun_comp_config.direction);
            fire_directions.push(fire_direction);
        }

        fire_directions
    }

    fn get_missile_indices(
        weapon_config: &WeaponConfig,
        game_lib: &GameLib,
    ) -> Result<Vec<usize>, MyError> {
        let mut missile_indices = Vec::new();

        for gun_comp_config in weapon_config.gun_components.iter() {
            let gun_config = game_lib.get_gun_config(&gun_comp_config.config_name)?;
            let missile_index = game_lib.get_game_obj_config_index(&gun_config.missile)?;
            missile_indices.push(missile_index);
        }

        Ok(missile_indices)
    }
}
