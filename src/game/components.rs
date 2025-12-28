use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub dest: Option<Vec2>,
    pub move_enabled: bool,
}

#[derive(Component)]
pub struct AIComponent;

impl PlayerComponent {
    pub fn new() -> Self {
        PlayerComponent {
            dest: None,
            move_enabled: false,
        }
    }
}
