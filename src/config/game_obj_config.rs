use crate::config::*;
use crate::game::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize)]
pub struct GameObjConfig {
    pub name: String,
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub obj_type: GameObjType,
    pub side: GameObjSide,
    pub speed: f32,
    pub collide_span: f32,
    pub weapon_config: Option<WeaponConfig>,
}
