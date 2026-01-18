use crate::config::*;
use crate::game::*;
use crate::misc::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize)]
pub struct PlayConfig {
    pub frame_count: usize,
    pub frames_per_second: usize,
}

#[derive(Debug, Resource, Deserialize)]
pub struct GameObjConfig {
    pub name: String,
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub obj_type: GameObjType,
    pub side: GameObjSide,
    pub speed: f32,
    pub hp: Option<f32>,
    pub collide_span: f32,
    pub weapon_config: Option<WeaponConfig>,
    pub damage: Option<f32>,
    pub play_config: Option<PlayConfig>,
    pub explosion: Option<String>,
}

impl GameObjConfig {
    pub fn size(&self) -> Vec2 {
        arr_to_vec2(&self.size)
    }
}
