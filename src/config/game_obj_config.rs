use crate::config::*;
use crate::misc::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum GameObjSide {
    Player,
    AI,
    Neutral,
}

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
    pub on_death_actions: Vec<OnDeathAction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MissileConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub side: GameObjSide,
    pub speed: f32,
    pub collide_span: f32,
    pub damage: f32,
    pub damage_range: f32,
    pub features: Vec<MissileFeature>,
    pub on_death_actions: Vec<OnDeathAction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayFrameConfig {
    pub image: String,
    pub size: [f32; 2],
    pub z: f32,
    pub frame_count: usize,
    pub frames_per_second: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub enum OnDeathAction {
    DoDamage,
    PlayFrame(String),
    Phaseout(f32),
}

#[derive(Debug, Clone, Deserialize)]
pub enum MissileFeature {
    Guided(EnemySearchConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemySearchConfig {
    pub search_span: f32,
    pub search_wait_duration: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum GameObjConfig {
    Tile(TileConfig),
    Bot(BotConfig),
    Missile(MissileConfig),
    PlayFrame(PlayFrameConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamedGameObjConfig {
    pub name: String,
    pub config: GameObjConfig,
}

impl NamedGameObjConfig {
    #[inline]
    pub fn is_tile(&self) -> bool {
        match &self.config {
            GameObjConfig::Tile(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_bot(&self) -> bool {
        match &self.config {
            GameObjConfig::Bot(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_ai_bot(&self) -> bool {
        match &self.config {
            GameObjConfig::Bot(config) => config.side == GameObjSide::AI,
            _ => false,
        }
    }

    #[inline]
    pub fn is_player(&self) -> bool {
        match &self.config {
            GameObjConfig::Bot(config) => config.side == GameObjSide::Player,
            _ => false,
        }
    }

    #[inline]
    pub fn is_missile(&self) -> bool {
        match &self.config {
            GameObjConfig::Missile(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_play_frame(&self) -> bool {
        match &self.config {
            GameObjConfig::PlayFrame(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_transient(&self) -> bool {
        match &self.config {
            GameObjConfig::Missile(_) | GameObjConfig::PlayFrame(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn tile_config(&self) -> Result<&TileConfig, MyError> {
        match &self.config {
            GameObjConfig::Tile(cfg) => Ok(cfg),
            _ => {
                let msg = "Not a Tile".to_string();
                debug!(msg);
                Err(MyError::Other(msg))
            }
        }
    }

    #[inline]
    pub fn bot_config(&self) -> Result<&BotConfig, MyError> {
        match &self.config {
            GameObjConfig::Bot(cfg) => Ok(cfg),
            _ => {
                let msg = "Not a Bot".to_string();
                debug!(msg);
                Err(MyError::Other(msg))
            }
        }
    }

    #[inline]
    pub fn missile_config(&self) -> Result<&MissileConfig, MyError> {
        match &self.config {
            GameObjConfig::Missile(cfg) => Ok(cfg),
            _ => {
                let msg = "Not a Missile".to_string();
                debug!(msg);
                Err(MyError::Other(msg))
            }
        }
    }

    #[inline]
    pub fn play_frame_config(&self) -> Result<&PlayFrameConfig, MyError> {
        match &self.config {
            GameObjConfig::PlayFrame(cfg) => Ok(cfg),
            _ => {
                let msg = "Not a Explosion".to_string();
                debug!(msg);
                Err(MyError::Other(msg))
            }
        }
    }

    pub fn get_on_death_actions(&self) -> Result<&Vec<OnDeathAction>, MyError> {
        match &self.config {
            GameObjConfig::Bot(cfg) => Ok(&cfg.on_death_actions),
            GameObjConfig::Missile(cfg) => Ok(&cfg.on_death_actions),
            _ => {
                let msg = "There is no on-death actions".to_string();
                error!(msg);
                Err(MyError::Other(msg))
            }
        }
    }
}
