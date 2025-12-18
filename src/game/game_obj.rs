use crate::game::game_map::*;
use crate::res::game_obj_config::*;
use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct GameObj {
    pub config_name: String,
    pub pos: Vec2,
    pub map_pos: MapPos,
    pub direction: Vec2,
    pub side: GameObjSide,
    pub obj_type: GameObjType,
    pub collide_span: f32,
    pub speed: f32,
    pub hp: Option<f32>,
}
