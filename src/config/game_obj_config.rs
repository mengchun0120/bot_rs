use crate::config::gun_config::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum GameObjType {
    Tile,
    Bot,
    Missile,
    Effect,
}

#[derive(Debug, Resource, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum GameObjSide {
    Player,
    AI,
    Neutral,
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
    pub collide_span: f32,
    pub gun_configs: Option<Vec<GunComponentConfig>>,
}
