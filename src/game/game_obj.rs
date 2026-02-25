use crate::config::*;
use crate::game_utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct MapPos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct GameObj {
    pub config_index: usize,
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
    pub side: GameObjSide,
    pub is_phaseout: bool,
    pub collide_span: f32,
    pub obj_type: GameObjType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameObjType {
    Bot,
    Tile,
    Missile,
    PlayFrame,
}

impl GameObj {
    #[inline]
    pub fn is_collidable(&self) -> bool {
        !self.is_phaseout && self.collide_span > 0.0
    }
}
