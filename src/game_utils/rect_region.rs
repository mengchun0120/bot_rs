use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct RectRegion {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

impl RectRegion {
    pub fn new(left: f32, bottom: f32, right: f32, top: f32) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }

    #[inline]
    pub fn covers(&self, pos: &Vec2) -> bool {
        pos.x >= self.left && pos.x < self.right && pos.y >= self.bottom && pos.y < self.top
    }
}
