use crate::ai::*;
use crate::config::*;
use crate::game_utils::*;
use bevy::prelude::*;
use rand::Rng;
use rand::rng;

#[derive(Component)]
pub struct ChaseShootAIEngine {
    config: ChaseShootAIConfig,
    stage: ChooseShootAIStage,
    stage_timer: Timer,
    keep_direction_timer: Timer,
}

#[derive(Debug, PartialEq, Eq)]
enum ChooseShootAIStage {
    Chasing,
    Shooting,
}

impl ChaseShootAIEngine {
    pub fn new(config: ChaseShootAIConfig) -> Self {
        let mut r = rng();
        let stage = if r.random_range(0.0..=1.0) < config.chase_prob {
            ChooseShootAIStage::Shooting
        } else {
            ChooseShootAIStage::Chasing
        };
        let stage_duration = match stage {
            ChooseShootAIStage::Chasing => config.chase_duration,
            ChooseShootAIStage::Shooting => config.shoot_duration,
        };
        let stage_timer = Timer::from_seconds(stage_duration, TimerMode::Repeating);
        let keep_direction_timer = Timer::from_seconds(config.keep_direction_duration, TimerMode::Repeating);

        Self {
            config,
            stage,
            stage_timer,
            keep_direction_timer,
        }
    }
}

impl AIEngine for ChaseShootAIEngine {
    fn run(
        &mut self,
        entity: &Entity,
        game_map: &mut GameMap,
        world_info: &WorldInfo,
        game_obj_lib: &mut GameObjLib,
        despawn_pool: &mut DespawnPool,
        game_lib: &GameLib,
        commands: &mut Commands,
        time: &Time,
    ) {
    }
}