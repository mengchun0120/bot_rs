use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub trait AIEngine: Send + Sync {
    fn run(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        move_comp: &mut MoveComponent,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
        time: &Time,
    );
}
