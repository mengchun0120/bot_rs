use bevy::prelude::*;

#[derive(Component)]
pub struct MoveComponent {
    pub speed: f32,
}

impl MoveComponent {
    pub fn new(speed: f32) -> Self {
        Self { speed: speed }
    }
}