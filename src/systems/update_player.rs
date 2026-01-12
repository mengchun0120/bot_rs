use crate::game::components::*;
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
    let Some(timer) = &mut q_player.1.move_timer else {
        return;
    };

    timer.tick(time.delta());
    if timer.is_finished() {
        q_player.1.clear_move_timer();
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

    game_map.update_origin(
        &new_pos,
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        despawn_pool.as_mut(),
        &mut commands,
    );

    let new_map_pos = game_map.get_map_pos(&new_pos);
    if new_map_pos != obj.map_pos {
        game_map.relocate(q_player.0, &obj.map_pos, &new_map_pos);
    }

    game_obj_lib.entry(q_player.0).and_modify(|obj| {
        obj.pos = new_pos;
        obj.map_pos = new_map_pos;
    });

    let screen_pos = game_map.get_screen_pos(&new_pos);
    q_player.2.translation.x = screen_pos.x;
    q_player.2.translation.y = screen_pos.y;

    if collide {
        q_player.1.clear_move_timer();
    }
}
