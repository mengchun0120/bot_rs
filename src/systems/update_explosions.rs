use crate::{game::*, game_utils::*};
use bevy::prelude::*;

pub fn update_explosions(
    mut q_explosion: Query<(Entity, &mut Sprite, &mut PlayComponent), With<ExplosionComponent>>,
    mut despawn_pool: ResMut<DespawnPool>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut play_comp) in q_explosion.iter_mut() {
        play_comp.timer.tick(time.delta());
        if play_comp.timer.is_finished()
            && let Some(atlas) = sprite.texture_atlas.as_mut()
        {
            if atlas.index < play_comp.last_index {
                atlas.index += 1;
            } else {
                despawn_pool.insert(entity);
            }
        }
    }
}
