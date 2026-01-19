use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;
use std::collections::HashSet;

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
    let obj_config = game_lib.get_game_obj_config(obj.config_index);

    let (collide, new_pos) = get_bot_new_pos(
        &q_player.0,
        &obj,
        game_map.as_ref(),
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

    let mut captured_missiles: HashSet<Entity> = HashSet::new();

    capture_missiles(
        &new_pos,
        obj_config.collide_span,
        obj_config.side,
        &mut captured_missiles,
        game_obj_lib.as_ref(),
        game_map.as_ref(),
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    );

    if !captured_missiles.is_empty() {
        explode_all(
            &mut captured_missiles,
            game_obj_lib.as_mut(),
            game_map.as_mut(),
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }

    update_origin(
        &new_pos,
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        game_map.as_mut(),
        despawn_pool.as_mut(),
        &mut commands,
    );
}

fn update_origin(
    origin: &Vec2,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    game_map: &mut GameMap,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let old_visible_region = game_map.get_visible_region();

    game_map.set_origin(origin);
    let new_visible_region = game_map.get_visible_region();

    hide_offscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_obj_lib,
        game_lib,
        game_map,
        despawn_pool,
        commands,
    );

    update_onscreen_screen_pos(
        &old_visible_region,
        &new_visible_region,
        game_obj_lib,
        game_map,
        commands,
    );

    show_newscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_obj_lib,
        game_map,
        commands,
    );
}

fn hide_offscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    game_map: &GameMap,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let offscreen_regions = old_visible_region.sub(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(config_index) = game_obj_lib.get(entity).map(|obj| obj.config_index) else {
            error!("Cannot find entity {:?} in GameObjLib", entity);
            return true;
        };
        let obj_type = game_lib.get_game_obj_config(config_index).obj_type;

        if obj_type == GameObjType::Missile || obj_type == GameObjType::Explosion {
            despawn_pool.insert(entity.clone());
            return true;
        }

        commands
            .entity(entity.clone())
            .entry::<Visibility>()
            .and_modify(|mut v| *v = Visibility::Hidden);

        true
    };

    game_map.run_on_regions(&offscreen_regions, func);
}

fn update_onscreen_screen_pos(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_obj_lib: &GameObjLib,
    game_map: &GameMap,
    commands: &mut Commands,
) {
    let onscreen_regions = old_visible_region.intersect(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = game_obj_lib.get(entity) else {
            return true;
        };
        let screen_pos = game_map.get_screen_pos(&obj.pos);
        commands
            .entity(entity.clone())
            .entry::<Transform>()
            .and_modify(move |mut t| {
                t.translation.x = screen_pos.x;
                t.translation.y = screen_pos.y;
            });

        true
    };

    game_map.run_on_regions(&onscreen_regions, func);
}

fn show_newscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_obj_lib: &GameObjLib,
    game_map: &GameMap,
    commands: &mut Commands,
) {
    let newscreen_regions = new_visible_region.sub(&old_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = game_obj_lib.get(entity) else {
            return true;
        };
        let screen_pos = game_map.get_screen_pos(&obj.pos);
        let mut entity = commands.entity(entity.clone());

        entity.entry::<Transform>().and_modify(move |mut t| {
            t.translation.x = screen_pos.x;
            t.translation.y = screen_pos.y;
        });

        entity.entry::<Visibility>().and_modify(|mut v| {
            *v = Visibility::Visible;
        });

        true
    };

    game_map.run_on_regions(&newscreen_regions, func);
}
