use bevy::prelude::*;

pub struct NewObj {
    pub config_name: String,
    pub pos: Vec2,
    pub direction: Vec2,
    pub speed: Option<f32>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct NewObjQueue(Vec<NewObj>);

impl NewObjQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
