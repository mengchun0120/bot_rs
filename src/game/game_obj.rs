use crate::config::*;
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
    pub is_phaseout: bool,
}
