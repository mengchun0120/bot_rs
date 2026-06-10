use crate::game::GameObj;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GameObjLib(HashMap<Entity, GameObj>);

impl GameObjLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn get(&self, entity: &Entity) -> Option<&GameObj> {
        self.0.get(entity)
    }

    #[inline]
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut GameObj> {
        self.0.get_mut(entity)
    }

    #[inline]
    pub fn remove(&mut self, entity: &Entity) {
        self.0.remove(entity);
    }

    #[inline]
    pub fn insert(&mut self, entity: Entity, obj: GameObj) {
        self.0.insert(entity, obj);
    }
}
