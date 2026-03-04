use bevy::prelude::*;

#[derive(Component)]
pub struct HPComponent {
    hp: f32,
}

impl HPComponent {
    pub fn new(hp: f32) -> Self {
        Self { hp }
    }

    #[inline]
    pub fn hp(&self) -> f32 {
        self.hp
    }

    pub fn update(&mut self, hp_delta: f32) {
        self.hp = (self.hp + hp_delta).max(0.0);
    }
}
