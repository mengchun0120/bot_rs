use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Deref, DerefMut)]
pub struct DespawnPool(pub HashSet<Entity>);

impl DespawnPool {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        for entity in self.0.iter() {
            let mut entity_cmd = commands.entity(entity.clone());
            entity_cmd.despawn_children();
            entity_cmd.despawn();
        }
        self.0.clear();
    }
}
