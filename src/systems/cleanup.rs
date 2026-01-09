use crate::game_utils::despawn_pool::*;
use bevy::prelude::*;

pub fn cleanup(
    mut commands: Commands,
    mut despawn_pool: ResMut<DespawnPool>,
) {
    despawn_pool.despawn(&mut commands);
}