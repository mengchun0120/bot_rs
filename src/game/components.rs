use crate::config::{weapon_config::*};
use crate::game_utils::game_lib::*;
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub move_timer: Option<Timer>,
}

#[derive(Component)]
pub struct AIComponent;

#[derive(Component)]
pub struct WeaponComponent {
    pub fire_timer: Timer,
    pub fire_points: Vec<Vec2>,
}

impl PlayerComponent {
    pub fn new() -> Self {
        PlayerComponent { move_timer: None }
    }

    pub fn reset_move_timer(&mut self, duration: f32) {
        self.move_timer = Some(Timer::from_seconds(duration, TimerMode::Once));
    }

    pub fn clear_move_timer(&mut self) {
        self.move_timer = None;
    }
}

impl WeaponComponent {
    pub fn new(weapon_config: &WeaponConfig, game_lib: &GameLib) -> Result<Self, MyError> {
        let fire_timer = Timer::from_seconds(weapon_config.fire_duration, TimerMode::Repeating);
        let fire_points = Self::get_fire_points(weapon_config, game_lib)?;

        Ok(Self { fire_timer, fire_points, })
    }

    fn get_fire_points(weapon_config: &WeaponConfig, game_lib: &GameLib) -> Result<Vec<Vec2>, MyError> {
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
}