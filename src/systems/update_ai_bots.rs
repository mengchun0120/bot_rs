use crate::ai::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai_bots(
    ai_bot_query: Query<Entity, (With<AIBot>, With<InView>)>,
    ai_comp_query: Query<&AIComponent>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut weapon_comp_query: Query<&mut WeaponComponent>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut hp_query: Query<&mut HPComponent>,
    mut world_info: ResMut<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for entity in ai_bot_query.iter() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Ok(ai_comp) = ai_comp_query.get(entity) else {
            error!("Cannot find AIComponent");
            continue;
        };

        match ai_comp.engine.cur_action() {
            AIAction::Chase => {
                if move_bot(
                    entity,
                    &mut move_comp_query,
                    &mut transform_query,
                    &mut visibility_query,
                    &mut hp_query,
                    world_info.as_mut(),
                    game_map.as_mut(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    despawn_pool.as_mut(),
                    &mut commands,
                    time.as_ref(),
                )
                .is_err()
                {
                    error!("Failed to move bot");
                }
            }
            AIAction::Shoot => {
                if try_shoot(
                    entity,
                    &move_comp_query,
                    &mut weapon_comp_query,
                    world_info.as_mut(),
                    game_map.as_mut(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    &mut commands,
                    time.as_ref(),
                )
                .is_err()
                {
                    error!("Failed to shoot");
                }
            }
            _ => {}
        }
    }
}
