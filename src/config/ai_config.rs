use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource, Copy, Clone)]
pub struct ChaseShootAiConfig {
    pub chase_prob: f32,
    pub chase_direction_keeptime: f32,
    pub chase_duration: f32,
    pub shoot_duration: f32,
    pub shoot_direction_keeptime: f32,
}

#[derive(Debug, Deserialize, Resource, Copy, Clone)]
pub enum AiConfig {
    ChaseShoot(ChaseShootAiConfig),
}
