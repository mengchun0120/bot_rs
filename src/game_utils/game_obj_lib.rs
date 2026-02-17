use crate::game::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GameObjLib(HashMap<Entity, GameObj>);

impl GameObjLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn get(&self, entity: &Entity) -> Result<&GameObj, MyError> {
        let Some(obj) = self.0.get(entity) else {
            let msg = format!("Cannot find GameObj {}", entity);
            error!(msg);
            return Err(MyError::NotFound(msg));
        };
        Ok(obj)
    }

    #[inline]
    pub fn get_mut(&mut self, entity: &Entity) -> Result<&mut GameObj, MyError> {
        let Some(obj) = self.0.get_mut(entity) else {
            let msg = format!("Cannot find GameObj {}", entity);
            error!(msg);
            return Err(MyError::NotFound(msg));
        };
        Ok(obj)
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
