use crate::config::*;
use crate::game_utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct GameObj {
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
    pub config: GameObjConfig,
}
