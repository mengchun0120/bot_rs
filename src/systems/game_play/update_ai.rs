use crate::game::{
    GameObjState,
    components::{AiBotComponent, AiComponent, InView, WeaponComponent},
};
use crate::game_utils::{GameInfo, GameLib, GameObjLib};
use bevy::prelude::*;

pub fn update_ai(
    mut aibot_query: Query<
        (
            Entity,
            &mut WeaponComponent,
            &mut AiComponent,
            &mut Transform,
        ),
        (With<AiBotComponent>, With<InView>),
    >,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    game_info: Res<GameInfo>,
    time: Res<Time>,
) {
    let Some(player_entity) = game_info.get_player() else {
        return;
    };
    let Ok(player) = game_obj_lib.get(&player_entity).cloned() else {
        return;
    };

    if player.state != GameObjState::Alive {
        return;
    }

    for (entity, mut weapon_comp, mut ai_comp, mut transform) in
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
            weapon_comp.as_mut(),
            &player.pos,
            game_lib.as_ref(),
            time.as_ref(),
        );
    }
}
