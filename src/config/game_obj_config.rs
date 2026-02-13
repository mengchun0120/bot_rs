use crate::config::*;
use crate::game::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TileConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub collide_span: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub side: GameObjSide,
    pub speed: f32,
    pub hp: f32,
    pub collide_span: f32,
    pub weapon_config: WeaponConfig,
    pub ai: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MissileConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub side: GameObjSide,
    pub speed: f32,
    pub collide_span: f32,
    pub explosion: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExplosionConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub damage: f32,
    pub frame_count: usize,
    pub frames_per_second: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub enum GameObjConfig {
    Tile(TileConfig),
    Bot(BotConfig),
    Missile(MissileConfig),
    Explosion(ExplosionConfig),
}