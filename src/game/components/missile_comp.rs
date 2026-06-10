use crate::config::{EnemySearchConfig, MissileConfig, MissileFeature, PierceConfig};
use crate::game::{GameObj, GameObjState, GameObjType, MoveResult, on_death, update_obj_pos};
use crate::game_utils::{
    DespawnPool, GameLib, GameMap, GameObjLib, NewObjQueue, RectRegion, WorldInfo,
};
use crate::misc::{
    MyError, check_collide_bounds, check_collide_obj, check_collide_objs, get_collide_region,
    get_rotation,
};
use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::collections::HashSet;

#[derive(Component)]
pub struct MissileComponent {
    pub alive_timer: Option<Timer>,
    pub enemy_search_ability: Option<EnemySearchAbility>,
    pub pierce_ability: Option<PierceAbility>,
}

pub struct EnemySearchAbility {
    search_timer: Timer,
    search_span: f32,
    potential_targets: Vec<Entity>,
    cur_target: Option<Entity>,
    initial_search: bool,
}

pub struct PierceAbility {
    pierced_entities: HashSet<Entity>,
    pierce_count: usize,
    max_pierce_count: usize,
    damage: f32,
}

impl MissileComponent {
    pub fn new(config: &MissileConfig) -> Self {
        let mut result = MissileComponent {
            alive_timer: None,
            enemy_search_ability: None,
            pierce_ability: None,
        };

        if let Some(alive_time) = config.alive_time {
            result.alive_timer = Some(Timer::from_seconds(alive_time, TimerMode::Once));
        }

        for feature in config.features.iter() {
            match feature {
                MissileFeature::Guided(cfg) => {
                    result.enemy_search_ability = Some(EnemySearchAbility::new(cfg));
                }
                MissileFeature::Pierce(cfg) => {
                    result.pierce_ability = Some(PierceAbility::new(cfg));
                }
            }
        }

        result
    }
}

impl EnemySearchAbility {
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
        let Some(target_pos) = game_obj_lib.get(target).map(|o| o.pos) else {
            let msg = "Failed to find obj in GameObjLib".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        };
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
        let Some(obj) = game_obj_lib.get(entity).cloned() else {
            let msg = "Failed to find obj in GameObjLib".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        };
        let search_region = RectRegion::new(
            obj.pos.x - self.search_span,
            obj.pos.y - self.search_span,
            obj.pos.x + self.search_span,
            obj.pos.y + self.search_span,
        );
        let map_region = game_map.get_region_from_rect(&search_region);
        self.potential_targets.clear();

        for e in game_map.map_iter(&map_region) {
            let Some(obj2) = game_obj_lib.get(&e) else {
                continue;
            };

            if obj2.state == GameObjState::Alive
                && obj2.side != obj.side
                && obj2.obj_type == GameObjType::Bot
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

        let Some(obj) = game_obj_lib.get(&target) else {
            let msg = "Failed to find obj in GameObjLib".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        };

        if obj.state != GameObjState::Alive {
            self.cur_target = None;
            return Ok(None);
        }

        Ok(Some(target))
    }
}

impl PierceAbility {
    pub fn new(config: &PierceConfig) -> Self {
        Self {
            pierced_entities: HashSet::new(),
            pierce_count: 0,
            max_pierce_count: config.max_pierce_count,
            damage: config.pierce_damage,
        }
    }

    pub fn move_obj(
        &mut self,
        entity: Entity,
        speed: f32,
        transform: &mut Transform,
        world_info: &WorldInfo,
        game_map: &mut GameMap,
        game_obj_lib: &mut GameObjLib,
        game_lib: &GameLib,
        new_obj_queue: &mut NewObjQueue,
        despawn_pool: &mut DespawnPool,
        commands: &mut Commands,
        time: &Time,
    ) -> Result<MoveResult, MyError> {
        if speed == 0.0 {
            return Ok(MoveResult::NotMoved);
        }

        let Some(obj) = game_obj_lib.get(&entity).cloned() else {
            let msg = "Failed to find obj in GameObjLib".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        };
        let new_pos = obj.pos + obj.direction * speed * time.delta_secs();

        if !world_info.check_pos_visible(&new_pos) {
            despawn_pool.add(entity, game_obj_lib)?;
            return Ok(MoveResult::NotMoved);
        }

        if self.check_collide(
            entity,
            &obj,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            new_obj_queue,
            commands,
        )? {
            on_death(
                entity,
                game_map,
                game_obj_lib,
                game_lib,
                new_obj_queue,
                commands,
            )?;
            despawn_pool.add(entity, game_obj_lib)?;
            Ok(MoveResult::Collided)
        } else {
            update_obj_pos(
                entity,
                new_pos,
                transform,
                world_info,
                game_map,
                game_obj_lib,
            )?;
            Ok(MoveResult::Moved(new_pos))
        }
    }

    fn check_collide(
        &mut self,
        entity: Entity,
        obj: &GameObj,
        world_info: &WorldInfo,
        game_map: &mut GameMap,
        game_obj_lib: &mut GameObjLib,
        game_lib: &GameLib,
        new_obj_queue: &mut NewObjQueue,
        commands: &mut Commands,
    ) -> Result<bool, MyError> {
        if check_collide_bounds(
            &obj.pos,
            obj.collide_span,
            world_info.world_width(),
            world_info.world_height(),
        ) {
            Ok(true)
        } else if self.pierce_count < self.max_pierce_count {
            self.check_pierce_objs(
                entity,
                obj,
                game_map,
                game_obj_lib,
                game_lib,
                new_obj_queue,
                commands,
            )?;
            Ok(false)
        } else if check_collide_objs(
            Some(entity),
            &obj.pos,
            obj.collide_span,
            game_lib.game_config.max_collide_span,
            game_map,
            game_obj_lib,
        ) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_pierce_objs(
        &mut self,
        entity: Entity,
        obj: &GameObj,
        game_map: &GameMap,
        game_obj_lib: &mut GameObjLib,
        game_lib: &GameLib,
        new_obj_queue: &mut NewObjQueue,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let collide_region = get_collide_region(
            &obj.pos,
            obj.collide_span,
            game_lib.game_config.max_collide_span,
            game_map,
        );
        let mut collide = false;

        for e in game_map.map_iter(&collide_region) {
            if e == entity || self.pierced_entities.contains(&e) {
                continue;
            }

            let Ok(obj2) = game_obj_lib.get_mut(&e) else {
                continue;
            };

            if obj2.is_collidable()
                && check_collide_obj(&obj.pos, obj.collide_span, &obj2.pos, obj2.collide_span)
            {
                self.pierced_entities.insert(e);
                collide = true;

                if obj2.obj_type == GameObjType::Bot && obj2.side != obj.side {
                    if let Some(hp) = obj2.hp {
                        let new_hp = (hp - self.damage).max(0.0);
                        obj2.hp = Some(new_hp);
                        if new_hp <= 0.0 {
                            on_death(e, game_map, game_obj_lib, game_lib, new_obj_queue, commands)?;
                        }
                    } else {
                        error!("Bot's hp is None");
                        continue;
                    }
                }
            }
        }

        if collide {
            self.pierce_count += 1;
        }

        Ok(())
    }
}
