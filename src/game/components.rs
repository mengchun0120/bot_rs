use crate::ai::*;
use crate::config::*;
use crate::game_utils::*;
use crate::misc::{my_error::*, utils::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveComponent {
    pub speed: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AIBot;

#[derive(Component)]
pub struct MissileComponent;

#[derive(Component)]
pub struct ExplosionComponent;

#[derive(Component)]
pub struct TileComponent;

#[derive(Component)]
pub struct PlayComponent {
    pub timer: Timer,
    pub last_index: usize,
}

#[derive(Component)]
pub struct AIComponent {
    pub engine: Box<dyn AIEngine>,
}

#[derive(Component)]
pub struct WeaponComponent {
    pub fire_timer: Timer,
    pub fire_points: Vec<Vec2>,
    pub fire_directions: Vec<Vec2>,
    pub missile_config_index: usize,
}

impl MoveComponent {
    pub fn new(speed: f32) -> Self {
        Self { speed: speed }
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
