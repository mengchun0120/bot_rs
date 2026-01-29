use std::time::Duration;

use crate::ai::*;
use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use rand::Rng;
use rand::rng;

#[derive(Component)]
pub struct ChaseShootAIEngine {
    config: ChaseShootAIConfig,
    action: AIAction,
    action_timer: Timer,
    keep_direction_timer: Timer,
    redirect_needed: bool,
}

impl ChaseShootAIEngine {
    pub fn new(
        config: ChaseShootAIConfig,
        entity: &Entity,
        player_pos: &Vec2,
        game_map: &mut GameMap,
        world_info: &WorldInfo,
        game_obj_lib: &mut GameObjLib,
        despawn_pool: &mut DespawnPool,
        game_lib: &GameLib,
    ) -> Self {
        let (action, action_duration) = Self::get_action(&config);
        let action_timer = Timer::from_seconds(action_duration, TimerMode::Repeating);
        let keep_direction_timer =
            Timer::from_seconds(config.keep_direction_duration, TimerMode::Repeating);
        let engine = Self {
            config,
            action,
            action_timer,
            keep_direction_timer,
            redirect_needed: false,
        };

        engine
    }

    fn run_chase(
        &mut self,
        entity: &Entity,
        move_comp: &mut MoveComponent,
        player_pos: &Vec2,
        game_obj_lib: &mut GameObjLib,
    ) {
    }

    fn run_shoot(
        &mut self,
        entity: &Entity,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_obj_lib: &mut GameObjLib,
    ) {
    }

    fn reset_action(
        &mut self,
        entity: &Entity,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_obj_lib: &mut GameObjLib,
    ) {
        let (action, action_duration) = Self::get_action(&self.config);
        self.action = action;
        self.action_timer
            .set_duration(Duration::from_secs_f32(action_duration));
    }

    fn get_action(config: &ChaseShootAIConfig) -> (AIAction, f32) {
        let mut r = rng();
        if r.random_range(0.0..=1.0) < config.chase_prob {
            (AIAction::Chase, config.chase_duration)
        } else {
            (AIAction::Shoot, config.shoot_duration)
        }
    }

    fn init_obj(
        entity: &Entity,
        player_pos: &Vec2,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        game_obj_lib: &mut GameObjLib,
    ) {
    }

}

impl AIEngine for ChaseShootAIEngine {
    fn run(
        &mut self,
        entity: &Entity,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        game_obj_lib: &mut GameObjLib,
        player_pos: &Vec2,
        time: &Time,
    ) {
        self.action_timer.tick(time.delta());
        if self.action_timer.is_finished() {
            self.reset_action(entity, move_comp, weapon_comp, player_pos, game_obj_lib);
        } else {
            match self.action {
                AIAction::Chase => {
                    self.run_chase(entity, move_comp, player_pos, game_obj_lib);
                }
                AIAction::Shoot => {
                    self.run_shoot(entity, weapon_comp, player_pos, game_obj_lib);
                }
                _ => {}
            };
        }
    }
}
