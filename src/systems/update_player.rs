use crate::game::components::*;
use crate::game_utils::{game_lib::*, game_map::*, game_obj_lib::*};
use crate::misc::utils::*;
use bevy::prelude::*;

pub fn update_player(
    mut q_player: Single<(Entity, &mut PlayerComponent, &mut Transform)>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Some(dest) = q_player.1.dest else {
        return;
    };
    let Some(obj) = game_obj_lib.get(&q_player.0).cloned() else {
        error!("Cannot find player in GameObjLib");
        return;
    };

    if is_close(&obj.pos, &dest) {
        q_player.1.dest = None;
        return;
    }

    let (collide, new_pos) = game_map.get_bot_new_pos(
        &q_player.0,
        &obj,
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        time.as_ref(),
    );

    game_map.update_origin(&new_pos, game_obj_lib.as_ref(), &mut commands);

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
        q_player.1.dest = None;
    }
}
