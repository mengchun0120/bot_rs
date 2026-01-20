use std::time::Duration;

use crate::config::*;
use crate::game_utils::*;
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    move_enabled: bool,
    move_timer: Timer,
}

#[derive(Component)]
pub struct AIComponent;

#[derive(Component)]
pub struct MissileComponent;

#[derive(Component)]
pub struct ExplosionComponent;

#[derive(Component)]
pub struct PlayComponent {
    pub timer: Timer,
    pub last_index: usize,
}

#[derive(Component)]
pub struct WeaponComponent {
    pub fire_timer: Timer,
    pub fire_points: Vec<Vec2>,
    pub fire_directions: Vec<Vec2>,
    pub missile_config_index: usize,
}

impl PlayerComponent {
    pub fn new() -> Self {
        PlayerComponent {
            move_enabled: false,
            move_timer: Timer::from_seconds(0.0, TimerMode::Repeating),
        }
    }

    #[inline]
    pub fn move_enabled(&self) -> bool {
        self.move_enabled
    }

    #[inline]
    pub fn start_moving(&mut self, move_duration: Duration) {
        self.move_enabled = true;
        self.move_timer.set_duration(move_duration);
        self.move_timer.reset();
    }

    #[inline]
    pub fn update_move_timer(&mut self, delta: Duration) {
        self.move_timer.tick(delta);
    }

    #[inline]
    pub fn stop_moving(&mut self) {
        self.move_enabled = false;
    }
}

impl WeaponComponent {
    pub fn new(weapon_config: &WeaponConfig, game_lib: &GameLib) -> Result<Self, MyError> {
        let fire_timer = Timer::from_seconds(weapon_config.fire_duration, TimerMode::Repeating);
        let fire_points = Self::get_fire_points(weapon_config, game_lib)?;
        let fire_directions = Self::get_fire_directions(weapon_config);
        let missile_config_index = game_lib.get_game_obj_config_index(&weapon_config.missile)?;

        Ok(Self {
            fire_timer,
            fire_points,
            fire_directions,
            missile_config_index,
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
}

impl PlayComponent {
    pub fn new(play_config: &PlayConfig) -> Self {
        let frame_duration = 1.0 / play_config.frames_per_second as f32;
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            last_index: play_config.frame_count - 1,
        }
    }
}
