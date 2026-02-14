use crate::game::*;
use crate::game_utils::*;
use bevy::prelude::*;

pub fn process_key(
    mut player_query: Query<
        (Entity, &mut MoveComponent, &mut WeaponComponent),
        With<PlayerComponent>,
    >,
    key_input: Res<ButtonInput<KeyCode>>,
    world_info: Res<WorldInfo>,
    mut game_obj_lib: ResMut<GameObjLib>,
    game_lib: Res<GameLib>,
    mut new_obj_queue: ResMut<NewObjQueue>,
    time: Res<Time>,
) {
    let Ok((entity, mut move_comp, mut weapon_comp)) = player_query.single_mut() else {
        return;
    };

    if key_input.just_pressed(KeyCode::KeyF) || key_input.pressed(KeyCode::KeyF) {
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
    } else if key_input.just_pressed(KeyCode::KeyS) {
        move_comp.speed = 0.0;
    }
}
