use crate::game::components::*;
use crate::game_utils::{game_lib::*, game_map::*, game_obj_lib::*};
use bevy::prelude::*;

pub fn update_missiles(
    q_missile: Query<(Entity, &mut Transform), With<MissileComponent>>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform) in q_missile.iter() {
        let Some(obj) = game_obj_lib.get(&entity).cloned() else {
            error!("Cannot find entity in GameObjLib");
            continue;
        };
    }
}
