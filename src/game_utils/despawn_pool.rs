use crate::game::GameObjState;
use crate::game_utils::GameObjLib;
use crate::misc::MyError;
use crate::obj_missing_from_lib;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Deref, DerefMut)]
pub struct DespawnPool(pub HashSet<Entity>);

impl DespawnPool {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn add(&mut self, entity: Entity, game_obj_lib: &mut GameObjLib) -> Result<(), MyError> {
        let Some(obj) = game_obj_lib.get_mut(&entity) else {
            return obj_missing_from_lib!();
        };

        if obj.state == GameObjState::Dead {
            let msg = "Obj is already dead".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        }

        obj.state = GameObjState::Dead;
        self.0.insert(entity);

        Ok(())
    }
}
