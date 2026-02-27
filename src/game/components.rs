use crate::ai::*;
use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct AIBotComponent;

#[derive(Component)]
pub struct MissileComponent;

#[derive(Component)]
pub struct ExplosionComponent;

#[derive(Component)]
pub struct TileComponent;

#[derive(Component)]
pub struct InView;

#[derive(Component)]
pub struct MoveComponent {
    pub speed: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayoutComponent(Box<dyn Playout>);

#[derive(Component)]
pub struct AIComponent {
    pub engine: Box<dyn AIEngine>,
}

#[derive(Component)]
pub struct WeaponComponent {
    pub fire_timer: Timer,
    pub fire_points: Vec<Vec2>,
    pub fire_directions: Vec<Vec2>,
    pub missile_config_index: usize,
}

#[derive(Component)]
pub struct HPComponent {
    hp: f32,
}

#[derive(Component)]
pub struct EnemySearchComponent {
    search_timer: Timer,
    search_span: f32,
    potential_targets: Vec<Entity>,
    cur_target: Option<Entity>,
    initial_search: bool,
}

impl MoveComponent {
    pub fn new(speed: f32) -> Self {
        Self { speed: speed }
    }
}

impl WeaponComponent {
    pub fn new(weapon_config: &WeaponConfig, game_lib: &GameLib) -> Result<Self, MyError> {
        let missile_config_index =
            game_lib.get_game_obj_config_index(&weapon_config.missile_name)?;
        let fire_timer = Timer::from_seconds(weapon_config.fire_duration, TimerMode::Repeating);
        let fire_points = Self::get_fire_points(weapon_config, game_lib)?;
        let fire_directions = Self::get_fire_directions(weapon_config);

        Ok(Self {
            fire_timer,
            fire_points,
            fire_directions,
            missile_config_index,
        })
    }

    fn get_fire_points(
        weapon_config: &WeaponConfig,
        game_lib: &GameLib,
    ) -> Result<Vec<Vec2>, MyError> {
        let mut fire_points = Vec::new();

        for gun_comp_config in weapon_config.gun_components.iter() {
            let gun_config = game_lib.get_gun_config(&gun_comp_config.config_name)?;
            let gun_pos = arr_to_vec2(&gun_comp_config.pos);
            let gun_direction = arr_to_vec2(&gun_comp_config.direction);
            let local_fire_point = arr_to_vec2(&gun_config.fire_point);
            let fire_point = gun_pos + gun_direction.rotate(local_fire_point);
            fire_points.push(fire_point);
        }

        Ok(fire_points)
    }

    fn get_fire_directions(weapon_config: &WeaponConfig) -> Vec<Vec2> {
        let mut fire_directions = Vec::new();

        for gun_comp_config in weapon_config.gun_components.iter() {
            let fire_direction = arr_to_vec2(&gun_comp_config.direction);
            fire_directions.push(fire_direction);
        }

        fire_directions
    }
}

impl AIComponent {
    pub fn new(ai_config: &AIConfig) -> Self {
        let engine = match ai_config {
            AIConfig::ChaseShoot(config) => Box::new(ChaseShootAIEngine::new(*config)),
        };

        AIComponent { engine }
    }
}

impl HPComponent {
    pub fn new(hp: f32) -> Self {
        Self { hp }
    }

    #[inline]
    pub fn hp(&self) -> f32 {
        self.hp
    }

    pub fn update(&mut self, hp_delta: f32) {
        self.hp = (self.hp + hp_delta).max(0.0);
    }
}

impl PlayoutComponent {
    pub fn new<T: Playout + 'static>(playout: T) -> Self {
        Self(Box::new(playout))
    }
}

impl EnemySearchComponent {
    pub fn new(config: &EnemySearchConfig) -> Self {
        Self {
            search_timer: Timer::from_seconds(config.search_wait_duration, TimerMode::Repeating),
            search_span: config.search_span,
            potential_targets: Vec::new(),
            cur_target: None,
            initial_search: true,
        }
    }

    pub fn update(
        &mut self,
        entity: &Entity,
        transform: &mut Transform,
        game_map: &GameMap,
        game_obj_lib: &mut GameObjLib,
        time: &Time,
    ) -> Result<(), MyError> {
        if let Some(target) = self.check_target_available(game_obj_lib)? {
            self.update_with_target(entity, &target, transform, game_obj_lib)?;
        } else {
            if self.initial_search || self.search_timer.tick(time.delta()).is_finished() {
                self.find_target(entity, transform, game_map, game_obj_lib)?;
            }
        }

        Ok(())
    }

    fn update_with_target(
        &mut self,
        entity: &Entity,
        target: &Entity,
        transform: &mut Transform,
        game_obj_lib: &mut GameObjLib,
    ) -> Result<(), MyError> {
        let target_pos = game_obj_lib.get(target).map(|o| o.pos)?;
        let obj = game_obj_lib.get_mut(entity)?;

        obj.direction = (target_pos - obj.pos).normalize();
        transform.rotation = get_rotation(&obj.direction);

        Ok(())
    }

    fn find_target(
        &mut self,
        entity: &Entity,
        transform: &mut Transform,
        game_map: &GameMap,
        game_obj_lib: &mut GameObjLib,
    ) -> Result<(), MyError> {
        let obj = game_obj_lib.get(entity).cloned()?;
        let search_region = RectRegion::new(
            obj.pos.x - self.search_span,
            obj.pos.y - self.search_span,
            obj.pos.x + self.search_span,
            obj.pos.y + self.search_span,
        );
        let map_region = game_map.get_region_from_rect(&search_region);
        self.potential_targets.clear();

        for e in game_map.map_iter(&map_region) {
            let Ok(obj2) = game_obj_lib.get(&e) else {
                continue;
            };

            if obj2.state == GameObjState::Alive
                && obj2.side != obj.side
                && obj2.obj_type != GameObjType::Bot
                && search_region.covers(&obj2.pos)
            {
                self.potential_targets.push(e);
            };

            if obj2.side != obj.side && search_region.covers(&obj2.pos) {}
        }

        let mut r = rand::rng();
        if let Some(target) = self.potential_targets.choose(&mut r).cloned() {
            self.cur_target = Some(target);
            self.search_timer.reset();
            self.update_with_target(entity, &target, transform, game_obj_lib)?;
        }

        self.initial_search = false;

        Ok(())
    }

    fn check_target_available(
        &mut self,
        game_obj_lib: &GameObjLib,
    ) -> Result<Option<Entity>, MyError> {
        let Some(target) = self.cur_target else {
            return Ok(None);
        };

        let obj = game_obj_lib.get(&target)?;

        if obj.state != GameObjState::Alive {
            self.cur_target = None;
            return Ok(None);
        }

        Ok(Some(target))
    }
}
