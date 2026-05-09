use crate::ai::{AiEngine, ChaseShootAiEngine};
use crate::config::AiConfig;
use bevy::prelude::*;

#[derive(Component)]
pub struct AiComponent {
    pub engine: Box<dyn AiEngine>,
}

impl AiComponent {
    pub fn new(ai_config: &AiConfig) -> Self {
        let engine = match ai_config {
            AiConfig::ChaseShoot(config) => Box::new(ChaseShootAiEngine::new(*config)),
        };

        AiComponent { engine }
    }
}
