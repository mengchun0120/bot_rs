use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn update_player(
    player_query: Single<(Entity, &mut MoveComponent), With<Player>>,
    mut obj_query: Query<&mut GameObj>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut hp_query: Query<&mut HPComponent>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if player_query.1.speed == 0.0 {
        return;
    }

    let Ok(obj) = obj_query.get(player_query.0).cloned() else {
        error!("Cannot find GameObj");
        return;
    };

    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let new_pos = obj.pos + obj.direction * player_query.1.speed * time.delta_secs();

    if !check_collide(
        &player_query.0,
        &new_pos,
        obj_config.collide_span,
        game_map.as_ref(),
        world_info.as_ref(),
        &obj_query,
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    ) {
        update_obj_pos(
            player_query.0,
            new_pos,
            game_map.as_mut(),
            world_info.as_ref(),
            &mut obj_query,
            &mut transform_query,
        );

        update_origin(
            &new_pos,
            game_map.as_mut(),
            world_info.as_mut(),
            &obj_query,
            &mut transform_query,
            &mut visibility_query,
            game_lib.as_ref(),
            despawn_pool.as_mut(),
        );
    }

    let mut captured_missiles: HashSet<Entity> = HashSet::new();

    capture_missiles(
        &new_pos,
        obj_config.collide_span,
        obj_config.side,
        &mut captured_missiles,
        game_map.as_ref(),
        world_info.as_ref(),
        &obj_query,
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    );

    if !captured_missiles.is_empty() {
        explode_all(
            &mut captured_missiles,
            game_map.as_mut(),
            world_info.as_mut(),
            &obj_query,
            &mut hp_query,
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }
}

fn update_origin(
    origin: &Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let old_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    world_info.set_origin(origin);
    let new_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    hide_offscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        obj_query,
        visibility_query,
        game_lib,
        despawn_pool,
    );

    update_onscreen_screen_pos(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        obj_query,
        transform_query,
    );

    show_newscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        obj_query,
        transform_query,
        visibility_query,
    );
}

fn hide_offscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    obj_query: &Query<&mut GameObj>,
    visibility_query: &mut Query<&mut Visibility>,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
) {
    let offscreen_regions = old_visible_region.sub(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let Ok(mut visibility) = visibility_query.get_mut(*entity) else {
            error!("Cannot find Visibility");
            return true;
        };
        let obj_type = game_lib.get_game_obj_config(obj.config_index).obj_type;

        if obj_type == GameObjType::Missile || obj_type == GameObjType::Explosion {
            despawn_pool.insert(*entity);
            return true;
        }

        *visibility = Visibility::Hidden;

        true
    };

    game_map.run_on_regions(&offscreen_regions, func);
}

fn update_onscreen_screen_pos(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
) {
    let onscreen_regions = old_visible_region.intersect(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let Ok(mut transform) = transform_query.get_mut(*entity) else {
            error!("Cannot find Transform");
            return true;
        };
        let screen_pos = world_info.get_screen_pos(&obj.pos);

        transform.translation.x = screen_pos.x;
        transform.translation.y = screen_pos.y;

        true
    };

    game_map.run_on_regions(&onscreen_regions, func);
}

fn show_newscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
) {
    let newscreen_regions = new_visible_region.sub(&old_visible_region);
    let func = |entity: &Entity| -> bool {
        let Ok(obj) = obj_query.get(*entity) else {
            return true;
        };
        let Ok(mut transform) = transform_query.get_mut(*entity) else {
            return true;
        };
        let Ok(mut visibility) = visibility_query.get_mut(*entity) else {
            return true;
        };
        let screen_pos = world_info.get_screen_pos(&obj.pos);

        transform.translation.x = screen_pos.x;
        transform.translation.y = screen_pos.y;

        *visibility = Visibility::Visible;

        true
    };

    game_map.run_on_regions(&newscreen_regions, func);
}
