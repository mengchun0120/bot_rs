use crate::misc::*;
use bevy::prelude::*;

pub trait Playout: Send + Sync {
    fn update(&mut self, sprite: &mut Sprite, time: &Time) -> Result<bool, MyError>;
}

pub struct FramePlay {
    timer: Timer,
    last_index: usize,
}

pub struct Phaseout {
    duration: f32,
    timer: Timer,
}

impl FramePlay {
    pub fn new(frame_duration: f32, last_index: usize) -> Self {
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            last_index,
        }
    }
}

impl Playout for FramePlay {
    fn update(&mut self, sprite: &mut Sprite, time: &Time) -> Result<bool, MyError> {
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
    fn update(&mut self, sprite: &mut Sprite, time: &Time) -> Result<bool, MyError> {
        self.timer.tick(time.delta());
        if !self.timer.is_finished() {
            let alpha = 1.0 - self.timer.elapsed_secs() / self.duration;
            sprite.color.set_alpha(alpha);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
