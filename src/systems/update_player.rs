use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_player(
    player_query: Single<Entity, With<Player>>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut hp_query: Query<&mut HPComponent>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok(result) = move_bot(
        player_query.entity(),
        &mut move_comp_query,
        &mut transform_query,
        &mut visibility_query,
        &mut hp_query,
        world_info.as_mut(),
        game_map.as_mut(),
        game_obj_lib.as_mut(),
        game_lib.as_ref(),
        new_obj_queue.as_mut(),
        despawn_pool.as_mut(),
        &mut commands,
        time.as_ref(),
    ) else {
        return;
    };

    if let MoveResult::Moved(pos) = result {
        update_origin(
            pos,
            &mut transform_query,
            &mut visibility_query,
            world_info.as_mut(),
            game_map.as_mut(),
            game_obj_lib.as_ref(),
            game_lib.as_ref(),
            despawn_pool.as_mut(),
            &mut commands,
        );
    }
}

fn update_origin(
    origin: Vec2,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &mut Query<&mut Visibility>,
    world_info: &mut WorldInfo,
    game_map: &mut GameMap,
    game_obj_lib: &GameObjLib,
    game_lib: &GameLib,
    despawn_pool: &mut DespawnPool,
    commands: &mut Commands,
) {
    let old_visible_region = world_info.visible_region().clone();

    world_info.set_origin(&origin);
    let new_visible_region = world_info.visible_region();
    let region = game_map.get_region(
        old_visible_region.left.min(new_visible_region.left),
        old_visible_region.bottom.min(new_visible_region.bottom),
        old_visible_region.right.max(new_visible_region.right),
        old_visible_region.top.max(new_visible_region.top),
    );

    for entity in game_map.map_iter(&region) {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get(&entity) else {
            error!("Cannot find GameObj {}", entity);
            continue;
        };
        let obj_config = game_lib.get_game_obj_config(obj.config_index);
        let Ok(mut transform) = transform_query.get_mut(entity) else {
            error!("Cannot find Transform {}", entity);
            continue;
        };
        let Ok(mut visibility) = visibility_query.get_mut(entity) else {
            error!("Cannot find Visibility {}", entity);
            continue;
        };

        if world_info.check_pos_visible(&obj.pos) {
            let screen_pos = world_info.get_screen_pos(&obj.pos);
            transform.translation.x = screen_pos.x;
            transform.translation.y = screen_pos.y;
            *visibility = Visibility::Visible;
            if obj_config.obj_type == GameObjType::Bot {
                commands.entity(entity).insert(InView);
            }
        } else {
            if obj_config.obj_type == GameObjType::Missile
                || obj_config.obj_type == GameObjType::Explosion
            {
                despawn_pool.insert(entity);
            } else {
                let screen_pos = world_info.get_screen_pos(&obj.pos);
                transform.translation.x = screen_pos.x;
                transform.translation.y = screen_pos.y;
                *visibility = Visibility::Hidden;
                if obj_config.obj_type == GameObjType::Bot {
                    commands.entity(entity).remove::<InView>();
                }
            }
        }
    }
}
