use crate::game::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Deref, DerefMut)]
pub struct GameObjLib(pub HashMap<Entity, GameObj>);

impl GameObjLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
