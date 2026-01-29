use crate::config::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub trait AIEngine {
    fn run(
        &mut self,
        entity: &Entity,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        game_obj_lib: &mut GameObjLib,
        player_pos: &Vec2,
        time: &Time,
    );
}
