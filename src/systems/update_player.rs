use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn update_player(
    mut player_query: Single<(Entity, &mut GameObj, &mut MoveComponent, &mut Transform), With<Player>>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut obj_query: Query<&mut GameObj>,
    mut hp_query: Query<&mut HPComponent>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if player_query.2.speed == 0.0 {
        return;
    }

    let obj_config = game_lib.get_game_obj_config(player_query.1.config_index);
    let new_pos = player_query.1.pos + player_query.1.direction * player_query.2.speed * time.delta_secs();

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

fn update_origin(
    origin: &Vec2,
    game_map: &mut GameMap,
    world_info: &mut WorldInfo,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let old_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    world_info.set_origin(origin);
    let new_visible_region = game_map.get_region_from_rect(world_info.visible_region());

    hide_offscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        game_obj_lib,
        game_lib,
        despawn_pool,
        commands,
    );

    update_onscreen_screen_pos(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        game_obj_lib,
        commands,
    );

    show_newscreen_objs(
        &old_visible_region,
        &new_visible_region,
        game_map,
        world_info,
        game_obj_lib,
        commands,
    );
}

fn hide_offscreen_objs(
    old_visible_region: &MapRegion,
    new_visible_region: &MapRegion,
    game_map: &GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
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
    game_map: &GameMap,
    world_info: &WorldInfo,
    game_obj_lib: &GameObjLib,
    commands: &mut Commands,
) {
    let onscreen_regions = old_visible_region.intersect(&new_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = game_obj_lib.get(entity) else {
            return true;
        };
        let screen_pos = world_info.get_screen_pos(&obj.pos);
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
    game_map: &GameMap,
    world_info: &WorldInfo,
    game_obj_lib: &GameObjLib,
    commands: &mut Commands,
) {
    let newscreen_regions = new_visible_region.sub(&old_visible_region);
    let func = |entity: &Entity| -> bool {
        let Some(obj) = game_obj_lib.get(entity) else {
            return true;
        };
        let screen_pos = world_info.get_screen_pos(&obj.pos);
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
