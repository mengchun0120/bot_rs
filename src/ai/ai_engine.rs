use crate::game_utils::*;
use bevy::prelude::*;

pub trait AIEngine {
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
    );
}