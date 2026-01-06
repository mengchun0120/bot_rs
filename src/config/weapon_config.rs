use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Resource)]
pub struct GunConfig {
    pub image: String,
    pub size: [f32; 2],
    pub fire_point: [f32; 2],
    pub z: f32,
}

#[derive(Debug, Deserialize, Resource)]
pub struct GunComponentConfig {
    pub config_name: String,
    pub pos: [f32; 2],
    pub direction: [f32; 2],
}

#[derive(Debug, Deserialize, Resource)]
pub struct WeaponConfig {
    pub missile: String,
    pub gun_components: Vec<GunComponentConfig>,
    pub fire_duration: f32,
}