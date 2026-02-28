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
    time: Res<Time>,
) {
    let Ok(player) = game_obj_lib.get(&player_query.entity()).cloned() else {
        return;
    };

    if player.state != GameObjState::Alive {
        return;
    }

    for (entity, mut move_comp, mut weapon_comp, mut ai_comp, mut transform) in
        aibot_query.iter_mut()
    {
        let Ok(obj) = game_obj_lib.get_mut(&entity) else {
            continue;
        };

        if obj.state != GameObjState::Alive {
            continue;
        }

        ai_comp.engine.run(
            obj,
            transform.as_mut(),
            move_comp.as_mut(),
            weapon_comp.as_mut(),
            &player.pos,
            game_lib.as_ref(),
            time.as_ref(),
        );
    }
}
