use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GunConfig {
    pub image: String,
    pub size: [f32; 2],
    pub fire_point: [f32; 2],
    pub z: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GunComponentConfig {
    pub config_name: String,
    pub pos: [f32; 2],
    pub direction: [f32; 2],
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponConfig {
    pub missile_name: String,
    pub gun_components: Vec<GunComponentConfig>,
    pub fire_duration: f32,
}
