use crate::{game::*, game_utils::*};
use bevy::prelude::*;

pub fn update_playout(
    mut playout_query: Query<(Entity, &mut PlayoutComponent)>,
    mut sprite_query: Query<&mut Sprite>,
    children_query: Query<&Children>,
    mut despawn_pool: ResMut<DespawnPool>,
    time: Res<Time>,
) {
    for (entity, mut playout_comp) in playout_query.iter_mut() {
        let Ok(still_exists) =
            playout_comp.update(entity, &mut sprite_query, &children_query, time.as_ref())
        else {
            continue;
        };

        if !still_exists {
            despawn_pool.insert(entity);
        }
    }
}
