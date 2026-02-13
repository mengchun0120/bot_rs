use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum GameObjSide {
    Player,
    AI,
}
