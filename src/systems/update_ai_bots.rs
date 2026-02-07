use crate::ai::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai_bots(
    ai_bot_query: Query<Entity, (With<AIBot>, With<InView>)>,
    mut obj_query: Query<&mut GameObj>,
    ai_comp_query: Query<&AIComponent>,
    mut move_comp_query: Query<&mut MoveComponent>,
    mut weapon_comp_query: Query<&mut WeaponComponent>,
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    mut hp_query: Query<&mut HPComponent>,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
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
                    &obj_query,
                    game_map.as_mut(),
                    world_info.as_mut(),
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
