use crate::config::*;
use crate::game::{components::*, *};
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::collections::HashSet;

#[derive(Component)]
pub struct MissileComponent {
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
    pub fn new(features: &Vec<MissileFeature>) -> Self {
        let mut result = MissileComponent {
            enemy_search_ability: None,
            pierce_ability: None,
        };

        for feature in features.iter() {
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

        let obj = game_obj_lib.get(&target)?;

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
        hp_query: &mut Query<&mut HPComponent>,
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

        let obj = game_obj_lib.get(&entity).cloned()?;
        let new_pos = obj.pos + obj.direction * speed * time.delta_secs();

        if !world_info.check_pos_visible(&new_pos) {
            despawn_pool.add(entity, game_obj_lib)?;
            return Ok(MoveResult::NotMoved);
        }

        if self.check_collide(
            entity,
            &obj,
            hp_query,
            world_info,
            game_map,
            game_obj_lib,
            game_lib,
            new_obj_queue,
            commands,
        )? {
            on_death(
                entity,
                hp_query,
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
        hp_query: &mut Query<&mut HPComponent>,
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
                hp_query,
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
        hp_query: &mut Query<&mut HPComponent>,
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

            let Ok(obj2) = game_obj_lib.get(&e) else {
                continue;
            };

            if obj2.is_collidable()
                && check_collide_obj(&obj.pos, obj.collide_span, &obj2.pos, obj2.collide_span)
            {
                self.pierced_entities.insert(e);
                collide = true;

                if obj2.obj_type == GameObjType::Bot && obj2.side != obj.side {
                    if let Ok(mut hp) = hp_query.get_mut(e) {
                        hp.update(-self.damage);
                        if hp.hp() <= 0.0 {
                            on_death(
                                e,
                                hp_query,
                                game_map,
                                game_obj_lib,
                                game_lib,
                                new_obj_queue,
                                commands,
                            )?;
                        }
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