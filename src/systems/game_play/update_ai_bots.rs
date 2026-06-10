use crate::ai::AiAction;
use crate::game::{
    GameObjState, MoveResult,
    components::{AiBotComponent, AiComponent, InView, WeaponComponent},
    move_bot, try_shoot,
};
use crate::game_utils::{DespawnPool, GameLib, GameMap, GameObjLib, NewObjQueue, WorldInfo};
use bevy::prelude::*;

pub fn update_ai_bots(
    mut ai_bot_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Visibility,
            &mut WeaponComponent,
            &AiComponent,
        ),
        (With<AiBotComponent>, With<InView>),
    >,
    world_info: Res<WorldInfo>,
    mut game_map: ResMut<GameMap>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    mut despawn_pool: ResMut<DespawnPool>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut visibility, mut weapon_comp, ai_comp) in ai_bot_query.iter_mut()
    {
        let Some(obj) = game_obj_lib.get(&entity) else {
            continue;
        };

        if obj.state != GameObjState::Alive {
            continue;
        }

        match ai_comp.engine.cur_action() {
            AiAction::Chase => {
                match move_bot(
                    entity,
                    obj.speed.unwrap_or(0.0),
                    transform.as_mut(),
                    visibility.as_mut(),
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
                        if let Some(obj) = game_obj_lib.get_mut(&entity)
                            && obj.state == GameObjState::Alive
                        {
                            obj.speed = Some(0.0);
                        }
                    }
                    _ => {}
                }
            }
            AiAction::Shoot => {
                let _ = try_shoot(
                    entity,
                    obj.speed.unwrap_or(0.0),
                    weapon_comp.as_mut(),
                    world_info.as_ref(),
                    game_obj_lib.as_mut(),
                    game_lib.as_ref(),
                    new_obj_queue.as_mut(),
                    time.as_ref(),
                );
            }
            _ => {}
        }
    }
}
