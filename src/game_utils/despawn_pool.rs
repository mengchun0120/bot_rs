use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource)]
pub struct DespawnPool {
    pool: HashSet<Entity>,
}

impl DespawnPool {
    pub fn new() -> Self {
        Self {
            pool: HashSet::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.pool.insert(entity);
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        for entity in self.pool.iter() {
            let mut entity_cmd = commands.entity(entity.clone());
            entity_cmd.despawn_children();
            entity_cmd.despawn();
        }
        self.pool.clear();
    }
}