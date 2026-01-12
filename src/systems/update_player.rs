use crate::game::{components::*, game_actions::*};
use crate::game_utils::{despawn_pool::*, game_lib::*, game_map::*, game_obj_lib::*};
use bevy::prelude::*;

pub fn update_player(
    mut q_player: Single<(Entity, &mut PlayerComponent, &mut Transform)>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if !q_player.1.update_move_timer(time.as_ref()) {
        return;
    }

    let Some(obj) = game_obj_lib.get(&q_player.0).cloned() else {
        error!("Cannot find player in GameObjLib");
        return;
    };

    let (collide, new_pos) = game_map.get_bot_new_pos(
        &q_player.0,
        &obj,
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        time.as_ref(),
    );

    if collide {
        q_player.1.clear_move_timer();
    }

    update_obj_pos(
        q_player.0,
        &new_pos,
        game_obj_lib.as_mut(),
        game_map.as_mut(),
        q_player.2.as_mut(),
    );

    game_map.update_origin(
        &new_pos,
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        despawn_pool.as_mut(),
        &mut commands,
    );
}
