use crate::config::*;
use crate::game::*;
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
    DoDamage(DamageConfig),
    PlayFrame(String),
    Phaseout(f32),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DamageConfig {
    pub damage_range: f32,
    pub damage: f32,
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

impl GameObjConfig {
    pub fn basic_info(&self) -> (GameObjSide, f32, GameObjType) {
        match self {
            Self::Bot(cfg) => (cfg.side, cfg.collide_span, GameObjType::Bot),
            Self::Missile(cfg) => (cfg.side, cfg.collide_span, GameObjType::Missile),
            Self::PlayFrame(_) => (GameObjSide::Neutral, 0.0, GameObjType::PlayFrame),
            Self::Tile(cfg) => (GameObjSide::Neutral, cfg.collide_span, GameObjType::Tile),
        }
    }
}

impl NamedGameObjConfig {
    #[inline]
    pub fn is_bot(&self) -> bool {
        match &self.config {
            GameObjConfig::Bot(_) => true,
            _ => false,
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
