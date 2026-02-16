use crate::game_utils::*;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct GameObj {
    pub config_index: usize,
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
    pub is_phasing: bool,
}
