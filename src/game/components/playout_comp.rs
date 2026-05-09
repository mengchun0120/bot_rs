use crate::game::Playout;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayoutComponent(Box<dyn Playout>);

impl PlayoutComponent {
    pub fn new<T: Playout + 'static>(playout: T) -> Self {
        Self(Box::new(playout))
    }
}
