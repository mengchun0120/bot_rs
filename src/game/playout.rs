use crate::misc::*;
use bevy::prelude::*;

pub trait Playout: Send + Sync {
    fn update(
        &mut self,
        entity: Entity,
        sprite_query: &mut Query<&mut Sprite>,
        children_query: &Query<&Children>,
        time: &Time,
    ) -> Result<bool, MyError>;
}

pub struct PlayFrame {
    timer: Timer,
    last_index: usize,
}

pub struct Phaseout {
    duration: f32,
    timer: Timer,
}

impl PlayFrame {
    pub fn new(frames_per_second: usize, frame_count: usize) -> Self {
        let frame_duration = 1.0 / frames_per_second as f32;
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            last_index: frame_count - 1,
        }
    }
}

impl Playout for PlayFrame {
    fn update(
        &mut self,
        entity: Entity,
        sprite_query: &mut Query<&mut Sprite>,
        _: &Query<&Children>,
        time: &Time,
    ) -> Result<bool, MyError> {
        let Ok(mut sprite) = sprite_query.get_mut(entity) else {
            let msg = format!("Cannot find Sprite {}", entity);
            error!(msg);
            return Err(MyError::NotFound(msg));
        };
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            let msg = "No TextureAtlas found".to_string();
            error!(msg);
            return Err(MyError::Other(msg));
        };

        self.timer.tick(time.delta());
        if self.timer.is_finished() {
            if atlas.index < self.last_index {
                atlas.index += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }
}

impl Phaseout {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

impl Playout for Phaseout {
    fn update(
        &mut self,
        entity: Entity,
        sprite_query: &mut Query<&mut Sprite>,
        children_query: &Query<&Children>,
        time: &Time,
    ) -> Result<bool, MyError> {
        let Ok(mut sprite) = sprite_query.get_mut(entity) else {
            let msg = format!("Cannot find Sprite {}", entity);
            error!(msg);
            return Err(MyError::NotFound(msg));
        };

        self.timer.tick(time.delta());
        if !self.timer.is_finished() {
            let alpha = 1.0 - self.timer.elapsed_secs() / self.duration;
            sprite.color.set_alpha(alpha);

            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    let Ok(mut sprite) = sprite_query.get_mut(child) else {
                        continue;
                    };
                    sprite.color.set_alpha(alpha);
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
