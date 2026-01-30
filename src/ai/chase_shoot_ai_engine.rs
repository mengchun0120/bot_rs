use crate::ai::*;
use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use rand::{Rng, rng};
use std::time::Duration;

pub struct ChaseShootAIEngine {
    config: ChaseShootAIConfig,
    action: AIAction,
    action_timer: Timer,
    direction_keep_timer: Timer,
    directions: Vec<WeightedDirection>,
}

struct WeightedDirection {
    direction: Vec2,
    weight: f32,
}

const DIRECTION_WEIGHTS: [f32; 4] = [1.0, 1.0, 3.0, 5.0];
const TOTAL_WEIGHTS: f32 = 10.0;

impl ChaseShootAIEngine {
    pub fn new(
        config: ChaseShootAIConfig,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
    ) -> Self {
        let (action, action_duration, direction_keeptime) = Self::rand_action(&config);
        let mut engine = Self {
            config,
            action,
            action_timer: Timer::from_seconds(action_duration, TimerMode::Repeating),
            direction_keep_timer: Timer::from_seconds(direction_keeptime, TimerMode::Repeating),
            directions: Self::init_directions(),
        };

        engine.init(obj, transform, move_comp, weapon_comp, player_pos, game_lib);

        engine
    }

    fn run_chase(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
        time: &Time,
    ) {
        if move_comp.speed == 0.0 {
            self.reconfig_direction(obj, transform, move_comp, player_pos, game_lib);
        } else {
            self.check_direction_keep_timer(obj, transform, player_pos, time);
        }
    }

    fn run_shoot(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        player_pos: &Vec2,
        time: &Time,
    ) {
        self.check_direction_keep_timer(obj, transform, player_pos, time);
    }

    fn reset_action(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
    ) {
        let (action, action_duration, direction_keeptime) = Self::rand_action(&self.config);
        self.action = action;
        self.action_timer
            .set_duration(Duration::from_secs_f32(action_duration));
        self.action_timer.reset();
        self.direction_keep_timer
            .set_duration(Duration::from_secs_f32(direction_keeptime));
        self.direction_keep_timer.reset();

        Self::set_direction(obj, transform, (player_pos - obj.pos).normalize());

        match self.action {
            AIAction::Chase => {
                let obj_config = game_lib.get_game_obj_config(obj.config_index);
                move_comp.speed = obj_config.speed;
            }
            AIAction::Shoot => {
                move_comp.speed = 0.0;
                weapon_comp.fire_timer.reset();
            }
            _ => {}
        }
    }

    fn rand_action(config: &ChaseShootAIConfig) -> (AIAction, f32, f32) {
        let mut r = rng();
        if r.random_range(0.0..=1.0) < config.chase_prob {
            (
                AIAction::Chase,
                config.chase_duration,
                config.chase_direction_keeptime,
            )
        } else {
            (
                AIAction::Shoot,
                config.shoot_duration,
                config.shoot_direction_keeptime,
            )
        }
    }

    fn init_directions() -> Vec<WeightedDirection> {
        vec![
            WeightedDirection {
                direction: Vec2::new(1.0, 0.0),
                weight: 0.0,
            },
            WeightedDirection {
                direction: Vec2::new(0.0, 1.0),
                weight: 0.0,
            },
            WeightedDirection {
                direction: Vec2::new(-1.0, 0.0),
                weight: 0.0,
            },
            WeightedDirection {
                direction: Vec2::new(0.0, -1.0),
                weight: 0.0,
            },
        ]
    }

    #[inline]
    fn set_direction(obj: &mut GameObj, transform: &mut Transform, direction: Vec2) {
        transform.rotation = get_rotation(&direction);
        obj.direction = direction;
    }

    fn init(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
    ) {
        Self::set_direction(obj, transform, (player_pos - obj.pos).normalize());

        match self.action {
            AIAction::Chase => {
                let obj_config = game_lib.get_game_obj_config(obj.config_index);
                move_comp.speed = obj_config.speed;
            }
            AIAction::Shoot => {
                move_comp.speed = 0.0;
                weapon_comp.fire_timer.reset();
            }
            _ => {}
        }
    }

    fn reconfig_direction(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
    ) {
        self.weigh_sort_directions(&obj.pos, player_pos);
        Self::set_direction(obj, transform, self.choose_rand_direction());
        move_comp.speed = game_lib.get_game_obj_config(obj.config_index).speed;
        self.direction_keep_timer.reset();
    }

    fn weigh_sort_directions(&mut self, pos: &Vec2, player_pos: &Vec2) {
        let d = player_pos - pos;
        for wd in self.directions.iter_mut() {
            wd.weight = d.dot(wd.direction);
        }
        self.directions
            .sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    }

    fn choose_rand_direction(&self) -> Vec2 {
        let mut r = rng();
        let dice = r.random_range(0.0..TOTAL_WEIGHTS);
        let mut sum: f32 = 0.0;
        let mut idx = 0;
        for (i, w) in DIRECTION_WEIGHTS.iter().enumerate() {
            sum += *w;
            if dice <= sum {
                idx = i;
                break;
            }
        }
        self.directions[idx].direction
    }

    fn check_direction_keep_timer(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        player_pos: &Vec2,
        time: &Time,
    ) {
        self.direction_keep_timer.tick(time.delta());
        if self.direction_keep_timer.is_finished() {
            Self::set_direction(obj, transform, (player_pos - obj.pos).normalize());
        }
    }
}

impl AIEngine for ChaseShootAIEngine {
    fn run(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
        time: &Time,
    ) {
        self.action_timer.tick(time.delta());
        if self.action_timer.is_finished() {
            self.reset_action(obj, transform, move_comp, weapon_comp, player_pos, game_lib);
        } else {
            match self.action {
                AIAction::Chase => {
                    self.run_chase(obj, transform, move_comp, player_pos, game_lib, time);
                }
                AIAction::Shoot => {
                    self.run_shoot(obj, transform, player_pos, time);
                }
                _ => {}
            };
        }
    }
}
