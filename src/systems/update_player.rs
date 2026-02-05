use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn update_player(
    mut player_query: Single<
        (Entity, &mut MoveComponent),
        With<Player>,
    >,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut obj_query: Query<&mut GameObj>,
    mut hp_query: Query<&mut HPComponent>,
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
    let new_pos =
        obj.pos + obj.direction * player_query.1.speed * time.delta_secs();

    if !check_collide(
        &player_query.0,
        &new_pos,
        obj_config.collide_span,
        game_map.as_ref(),
        world_info.as_ref(),
        QueryMapperByMut::new(&obj_query),
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    ) {
        update_obj_pos(
            player_query.0,
            new_pos,
            game_map.as_mut(),
            world_info.as_ref(),
            &mut obj_query,
            player_query.3.as_mut(),
        );
        update_origin(
            &new_pos,
            game_map.as_mut(),
            world_info.as_mut(),
            game_obj_lib.as_ref(),
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
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
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    );

    if !captured_missiles.is_empty() {
        explode_all(
            &mut captured_missiles,
            game_obj_lib.as_mut(),
            game_map.as_mut(),
            world_info.as_mut(),
            &mut hp_query,
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }
}

fn update_origin<T, U, W>(
    origin: &Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    obj_mapper: &T,
    transform_mapper: &mut U,
    visibility_mapper: &mut W,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
)
where
    T: Mapper<Entity, GameObj>,
    U: MutMapper<Entity, Transform>,
    W: MutMapper<Entity, Visibility>,
{
    let old_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    world_info.set_origin(origin);
    let new_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    hide_offscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        obj_mapper,
        visibility_mapper,
        game_lib,
        despawn_pool,
    );

    update_onscreen_screen_pos(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        obj_mapper,
        transform_mapper,
    );

    show_newscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        obj_mapper,
        transform_mapper,
        visibility_mapper,
    );
}

fn hide_offscreen_objs<T, U>(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    obj_mapper: &T,
    visibility_mapper: &mut U,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
)
where
    T: Mapper<Entity, GameObj>,
    U: MutMapper<Entity, Visibility>,
{
    let offscreen_regions = old_visible_region.sub(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = obj_mapper.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let Some(visibility) = visibility_mapper.get(*entity) else {
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

fn update_onscreen_screen_pos<T, U>(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_mapper: &T,
    transform_mapper: &mut U,
)
where
    T: Mapper<Entity, GameObj>,
    U: MutMapper<Entity, Transform>
{
    let onscreen_regions = old_visible_region.intersect(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = obj_mapper.get(*entity) else {
            error!("Cannot find GameObj");
            return true;
        };
        let Some(transform) = transform_mapper.get(*entity) else {
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

fn show_newscreen_objs<T, U, W>(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    world_info: &WorldInfo,
    obj_mapper: &T,
    transform_mapper: &mut U,
    visibility_mapper: &mut W,
)
where
    T: Mapper<Entity, GameObj>,
    U: MutMapper<Entity, Transform>,
    W: MutMapper<Entity, Visibility>,
{
    let newscreen_regions = new_visible_region.sub(&old_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = obj_mapper.get(*entity) else {
            return true;
        };
        let Some(transform) = transform_mapper.get(*entity) else {
            return true;
        };
        let Some(visibility) = visibility_mapper.get(*entity) else {
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
