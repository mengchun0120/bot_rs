use crate::ai::AiAction;
use crate::game::{GameObj, components::WeaponComponent};
use crate::game_utils::GameLib;
use bevy::prelude::*;

pub trait AiEngine: Send + Sync {
    fn run(
        &mut self,
        obj: &mut GameObj,
        transform: &mut Transform,
        weapon_comp: &mut WeaponComponent,
        player_pos: &Vec2,
        game_lib: &GameLib,
        time: &Time,
    );

    fn cur_action(&self) -> AiAction;
}
