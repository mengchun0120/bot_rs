use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn update_ai(
    player_query: Single<Entity, With<PlayerComponent>>,
    mut aibot_query: Query<
        (
            Entity,
            &mut MoveComponent,
            &mut WeaponComponent,
            &mut AIComponent,
            &mut Transform,
        ),
        (With<AIBotComponent>, With<InView>),
    >,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    despawn_pool: Res<DespawnPool>,
    time: Res<Time>,
) {
    let Some(player_pos) = game_obj_lib.get(&player_query.entity()).map(|obj| obj.pos) else {
        error!("Cannot find Player");
        return;
    };

    for (entity, mut move_comp, mut weapon_comp, mut ai_comp, mut transform) in
        aibot_query.iter_mut()
    {
        if despawn_pool.contains(&entity) {
            continue;
        }

        let Some(obj) = game_obj_lib.get_mut(&entity) else {
            error!("Cannot find AIBot");
            continue;
        };

        ai_comp.engine.run(
            obj,
            transform.as_mut(),
            move_comp.as_mut(),
            weapon_comp.as_mut(),
            &player_pos,
            game_lib.as_ref(),
            time.as_ref(),
        );
    }
}
