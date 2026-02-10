use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_origin(
    player_query: Single<Entity, With<Player>>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    game_map: Res<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    game_obj_lib: Res<GameObjLib>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
) {
    let Some(player) = game_obj_lib.get(&player_query.entity()) else {
        error!("Cannot find player {}", player_query.entity());
        return;
    };
    let old_origin = world_info.origin();
    let old_visible_region = world_info.visible_region().clone();

    world_info.set_origin(&player.pos);
    if old_origin == world_info.origin() {
        return;
    }

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
