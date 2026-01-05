use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub move_timer: Option<Timer>,
}

#[derive(Component)]
pub struct AIComponent;

impl PlayerComponent {
    pub fn new() -> Self {
        PlayerComponent { move_timer: None }
    }

    pub fn reset_move_timer(&mut self, duration: f32) {
        self.move_timer = Some(Timer::from_seconds(duration, TimerMode::Once));
    }

    pub fn clear_move_timer(&mut self) {
        self.move_timer = None;
    }
}
