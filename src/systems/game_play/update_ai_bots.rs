use crate::ai::AiAction;
use crate::game::{
    GameObjState, MoveResult,
    components::{
        AiBotComponent, AiComponent, HpComponent, InView, MoveComponent, WeaponComponent,
    },
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
            &mut MoveComponent,
            &mut WeaponComponent,
            &AiComponent,
        ),
        (With<AiBotComponent>, With<InView>),
    >,
    mut hp_query: Query<&mut HpComponent>,
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
        if let Ok(obj) = game_obj_lib.get(&entity) {
            if obj.state != GameObjState::Alive {
                continue;
            }
        } else {
            continue;
        }

        match ai_comp.engine.cur_action() {
            AiAction::Chase => {
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
                        if let Ok(obj) = game_obj_lib.get(&entity)
                            && obj.state == GameObjState::Alive
                        {
                            move_comp.speed = 0.0;
                        }
                    }
                    _ => {}
                }
            }
            AiAction::Shoot => {
                let _ = try_shoot(
                    entity,
                    move_comp.speed,
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
