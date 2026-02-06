use crate::ai::*;
use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai_bots(
    player_query: Single<Entity, With<Player>>,
    mut aibot_query: Query<
        (
            Entity,
            &mut MoveComponent,
            &mut WeaponComponent,
            &mut AIComponent,
            &mut Transform,
        ),
        With<AIBot>,
    >,
    mut game_map: ResMut<GameMap>,
    mut world_info: ResMut<WorldInfo>,
    mut obj_query: Query<&mut GameObj>,
    game_lib: Res<GameLib>,
    despawn_pool: Res<DespawnPool>,
    time: Res<Time>,
) {
    let Ok(player_pos) = obj_query.get(player_query.entity()).map(|obj| obj.pos) else {
        error!("Cannot find Player");
        return;
    };

    for (entity, mut move_comp, mut weapon_comp, mut ai_comp, mut transform) in
        aibot_query.iter_mut()
    {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Ok(mut obj) = obj_query.get_mut(entity) else {
            error!("Cannot find GameObj");
            continue;
        };

        match ai_comp.engine.cur_action() {
            AIAction::Chase => {}
            AIAction::Shoot => {}
            _ => {}
        }
    }
}
