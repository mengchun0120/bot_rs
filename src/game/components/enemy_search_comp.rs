use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use rand::seq::IndexedRandom;

#[derive(Component)]
pub struct EnemySearchComponent {
    search_timer: Timer,
    search_span: f32,
    potential_targets: Vec<Entity>,
    cur_target: Option<Entity>,
    initial_search: bool,
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
