use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum GameObjType {
    Tile,
    Bot,
    Missile,
    Effect,
}
