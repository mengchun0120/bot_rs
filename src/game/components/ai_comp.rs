use crate::config::*;
use crate::ai::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct AIComponent {
    pub engine: Box<dyn AIEngine>,
}

impl AIComponent {
    pub fn new(ai_config: &AIConfig) -> Self {
        let engine = match ai_config {
            AIConfig::ChaseShoot(config) => Box::new(ChaseShootAIEngine::new(*config)),
        };

        AIComponent { engine }
    }
}