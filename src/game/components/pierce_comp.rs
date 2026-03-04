use crate::config::*;
use crate::game::{components::*, *};
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct PierceComponent {
    pierced_entities: HashSet<Entity>,
    pierce_count: usize,
    max_pierce_count: usize,
    damage: f32,
}

impl PierceComponent {
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
