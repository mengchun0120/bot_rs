use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_player(
    player_query: Single<Entity, With<Player>>,
    mut obj_query: Query<&mut GameObj>,
    mut move_comp_query: Query<&mut MoveComponent>,
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
    let Ok(result) = move_bot(
        player_query.entity(),
        &mut move_comp_query,
        &mut obj_query,
        &mut transform_query,
        &mut visibility_query,
        &mut hp_query,
        game_map.as_mut(),
        world_info.as_mut(),
        game_lib.as_ref(),
        despawn_pool.as_mut(),
        &mut commands,
        time.as_ref(),
    ) else {
        return;
    };

    if let MoveResult::Moved(pos) = result {
        update_origin(
            pos,
            &obj_query,
            &mut transform_query,
            &mut visibility_query,
            game_map.as_mut(),
            world_info.as_mut(),
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }
}

fn update_origin(
    origin: Vec2,
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let old_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    world_info.set_origin(&origin);
    let new_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    hide_offscreen_objs(
        &old_visible_region,
        &new_visible_region,
        obj_query,
        visibility_query,
        game_map,
        game_lib,
        despawn_pool,
        commands,
    );

    update_onscreen_screen_pos(
        &old_visible_region,
        &new_visible_region,
        obj_query,
        transform_query,
        game_map,
        world_info,
        despawn_pool,
    );

    show_newscreen_objs(
        &old_visible_region,
        &new_visible_region,
        obj_query,
        transform_query,
        visibility_query,
        game_map,
        world_info,
        game_lib,
        despawn_pool,
        commands,
    );
}

fn hide_offscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    obj_query: &Query<&mut GameObj>,
    visibility_query: &mut Query<&mut Visibility>,
    game_map: &GameMap,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let offscreen_regions = old_visible_region.sub(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        if despawn_pool.contains(entity) {
            return true;
        }

        let Ok(obj) = obj_query.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let Ok(mut visibility) = visibility_query.get_mut(*entity) else {
            error!("Cannot find Visibility");
            return true;
        };
        let obj_type = game_lib.get_game_obj_config(obj.config_index).obj_type;

        match obj_type {
            GameObjType::Bot => {
                *visibility = Visibility::Hidden;
                commands.entity(*entity).remove::<InView>();
            }
            GameObjType::Missile | GameObjType::Explosion => {
                despawn_pool.insert(*entity);
            }
            GameObjType::Tile => {
                *visibility = Visibility::Hidden;
            }
        }

        true
    };

    game_map.run_on_regions(&offscreen_regions, func);
}

fn update_onscreen_screen_pos(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
    game_map: &GameMap,
    world_info: &WorldInfo,
    despawn_pool: &DespawnPool,
) {
    let onscreen_regions = old_visible_region.intersect(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        if despawn_pool.contains(entity) {
            return true;
        }

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
    obj_query: &Query<&mut GameObj>,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
    game_map: &GameMap,
    world_info: &WorldInfo,
    game_lib: &GameLib,
    despawn_pool: &DespawnPool,
    commands: &mut Commands,
) {
    let newscreen_regions = new_visible_region.sub(&old_visible_region);
    let func = |entity: &Entity| -> bool {
        if despawn_pool.contains(entity) {
            return true;
        }

        let Ok(obj) = obj_query.get(*entity) else {
            return true;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);

        if obj_config.side == GameObjSide::Player {
            return true;
        }

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

        if obj_config.obj_type == GameObjType::Bot {
            commands.entity(*entity).insert(InView);
        }

        true
    };

    game_map.run_on_regions(&newscreen_regions, func);
}
