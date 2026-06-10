use crate::game::{
    GameObjState,
    components::{PlayerComponent, WeaponComponent},
    try_shoot,
};
use crate::game_utils::{GameLib, GameObjLib, NewObjQueue, WorldInfo};
use bevy::prelude::*;

pub fn process_key(
    mut player_query: Query<(Entity, &mut WeaponComponent), With<PlayerComponent>>,
    key_input: Res<ButtonInput<KeyCode>>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    time: Res<Time>,
) {
    let Ok((entity, mut weapon_comp)) = player_query.single_mut() else {
        return;
    };

    let Some(obj) = game_obj_lib.get_mut(&entity) else {
        error!("Failed to find obj in GameObjLib");
        return;
    };

    if obj.state != GameObjState::Alive {
        return;
    }

    let Some(speed) = obj.speed else {
        error!("speed is none");
        return;
    };

    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
        if try_shoot(
            entity,
            speed,
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
    } else if key_input.just_pressed(KeyCode::KeyS) {
        obj.speed = Some(0.0);
    }
}
