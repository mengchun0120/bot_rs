use crate::game::*;
use crate::game_utils::*;
use crate::misc::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn update_player(
    mut q_player: Single<(Entity, &mut MoveComponent, &mut Transform), With<PlayerComponent>>,
    game_lib: Res<GameLib>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if q_player.1.speed == 0.0 {
        return;
    }

    let Some(obj) = game_obj_lib.get(&q_player.0).cloned() else {
        error!("Cannot find player in GameObjLib");
        return;
    };
    let obj_config = game_lib.get_game_obj_config(obj.config_index);
    let new_pos = obj.pos + obj.direction * q_player.1.speed * time.delta_secs();

    let (collide, new_pos) = check_collide(
        &q_player.0,
        &obj.pos,
        &new_pos,
        &obj.direction,
        obj_config.collide_span,
        game_map.as_ref(),
        world_info.as_ref(),
        game_obj_lib.as_ref(),
        game_lib.as_ref(),
        despawn_pool.as_ref(),
    );

    update_obj_pos(
        q_player.0,
        &new_pos,
        game_map.as_mut(),
        world_info.as_ref(),
        game_obj_lib.as_mut(),
        q_player.2.as_mut(),
    );

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
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }

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
