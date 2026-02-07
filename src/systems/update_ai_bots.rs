use crate::ai::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai_bots(
    mut aibot_query: Query<
        (
            Entity,
            &mut MoveComponent,
            &mut WeaponComponent,
            &AIComponent,
        ),
        With<AIBot>,
    >,
    mut obj_query: Query<&mut GameObj>,
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
    for (entity, mut move_comp, mut weapon_comp, ai_comp) in aibot_query.iter_mut() {
        if despawn_pool.contains(&entity) {
            continue;
        }

        match ai_comp.engine.cur_action() {
            AIAction::Chase => {
                if move_bot(
                    entity,
                    move_comp.as_mut(),
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
                weapon_comp.fire_timer.tick(time.delta());
                if weapon_comp.fire_timer.is_finished() {
                    if shoot(
                        entity,
                        move_comp.speed,
                        weapon_comp.as_ref(),
                        &obj_query,
                        game_map.as_mut(),
                        world_info.as_mut(),
                        game_lib.as_ref(),
                        &mut commands,
                    )
                    .is_err()
                    {
                        error!("Failed to shoot");
                    }
                }
            }
            _ => {}
        }
    }
}
