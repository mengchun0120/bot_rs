use crate::ai::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai_bots(
    mut ai_bot_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Visibility,
            &mut MoveComponent,
            &mut WeaponComponent,
            &AIComponent,
        ),
        (With<AIBotComponent>, With<InView>),
    >,
    mut hp_query: Query<&mut HPComponent>,
    world_info: Res<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut visibility, mut move_comp, mut weapon_comp, ai_comp) in
        ai_bot_query.iter_mut()
    {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get(&entity).cloned() else {
            error!("Cannot find GameObj {}", entity);
            continue;
        };
        if obj.is_phaseout {
            continue;
        }

        match ai_comp.engine.cur_action() {
            AIAction::Chase => {
                match move_bot(
                    entity,
                    move_comp.speed,
                    transform.as_mut(),
                    visibility.as_mut(),
                    &mut hp_query,
                    world_info.as_ref(),
                    game_map.as_mut(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    new_obj_queue.as_mut(),
                    despawn_pool.as_mut(),
                    &mut commands,
                    time.as_ref(),
                ) {
                    Ok(MoveResult::Collided) => {
                        move_comp.speed = 0.0;
                    }
                    _ => {}
                }
            }
            AIAction::Shoot => {
                if try_shoot(
                    entity,
                    move_comp.speed,
                    weapon_comp.as_mut(),
                    world_info.as_ref(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    new_obj_queue.as_mut(),
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
