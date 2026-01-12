use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Deref, DerefMut)]
pub struct DespawnPool(pub HashSet<Entity>);

impl DespawnPool {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}
