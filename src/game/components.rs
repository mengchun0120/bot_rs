use crate::ai::*;
use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;

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
        }
    }

    pub fn update(
        &mut self,
        entity: &Entity,
        transform_query: &mut Query<&mut Transform>,
        game_map: &GameMap,
        game_obj_lib: &mut GameObjLib,
        despawn_pool: &DespawnPool,
        time: &Time,
    ) -> Result<(), MyError> {
        let obj = game_obj_lib.get_mut(entity)?;

        if let Some(target) = self.cur_target && !despawn_pool.contains(&target) {

        } else {

        }



        Ok(())
    }

    fn update_with_target(
        &mut self,
        entity: &Entity, 
        target: &Entity,
        transform_query: &mut Query<&mut Transform>,
        game_obj_lib: &mut GameObjLib,
    ) -> Result<(), MyError> {
        let target_pos = game_obj_lib.get(target).map(|o| o.pos)?;
        let obj = game_obj_lib.get_mut(entity)?;
        let Ok(mut transform) = transform_query.get_mut(*entity) else {
            let msg = format!("Cannot find Transform for {}", entity);
            error!(msg);
            return Err(MyError::Other(msg));
        };

        obj.direction = (target_pos - obj.pos).normalize();
        transform.rotation = get_rotation(&obj.direction);

        Ok(())
    }

    fn search_for_target(
        &mut self,
        entity: &Entity,
        transform_query: &mut Query<&mut Transform>,
        game_map: &GameMap,
        game_obj_lib: &mut GameObjLib,
        game_lib: &GameLib,
        despawn_pool: &DespawnPool,
    ) -> Result<(), MyError> {
        let obj = game_obj_lib.get(entity)?;
        let region = game_map.get_region(
            obj.pos.x - self.search_span,
            obj.pos.y - self.search_span,
            obj.pos.x + self.search_span,
            obj.pos.y + self.search_span,
        );

        self.potential_targets.clear();

        for e in game_map.map_iter(&region) {
            if despawn_pool.contains(&e) {
                continue;
            }

            let Ok(obj2) = game_obj_lib.get(&e) else {
                continue;
            };

            if obj2.is_phaseout {
                continue;
            }

            let Ok(config) = game_lib.get_game_obj_config(obj2.config_index).bot_config() else {
                continue;
            };

            

        }

        Ok(())
    }
}
