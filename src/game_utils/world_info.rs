use crate::game_utils::*;
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct WorldInfo {
    world_width: f32,
    world_height: f32,
    world_region: RectRegion,
    min_origin: Vec2,
    max_origin: Vec2,
    origin: Vec2,
    visible_span: Vec2,
    visible_region: RectRegion,
    max_collide_span: f32,
    player_pos: Option<Vec2>,
}

impl WorldInfo {
    pub fn new(
        world_width: f32,
        world_height: f32,
        window_width: f32,
        window_height: f32,
        visible_ext_size: f32,
        origin: &Vec2,
    ) -> Self {
        let mut world_info = Self {
            world_width,
            world_height,
            world_region: RectRegion::new(0.0, 0.0, world_width, world_height),
            min_origin: Vec2::new(window_width / 2.0, window_height / 2.0),
            max_origin: Vec2::new(
                world_width - window_width / 2.0,
                world_height - window_height / 2.0,
            ),
            origin: Vec2::default(),
            visible_span: Vec2::new(
                window_width / 2.0 + visible_ext_size,
                window_height / 2.0 + visible_ext_size,
            ),
            visible_region: RectRegion::default(),
            max_collide_span: 0.0,
            player_pos: None,
        };

        world_info.set_origin(&origin);

        world_info
    }

    #[inline]
    pub fn world_width(&self) -> f32 {
        self.world_width
    }

    #[inline]
    pub fn world_height(&self) -> f32 {
        self.world_height
    }

    #[inline]
    pub fn origin(&self) -> Vec2 {
        self.origin.clone()
    }

    #[inline]
    pub fn get_screen_pos(&self, pos: &Vec2) -> Vec2 {
        pos - self.origin
    }

    #[inline]
    pub fn viewport_to_world(&self, pos: &Vec2) -> Vec2 {
        pos + self.origin
    }

    pub fn set_origin(&mut self, origin: &Vec2) {
        self.origin.x = origin.x.clamp(self.min_origin.x, self.max_origin.x);
        self.origin.y = origin.y.clamp(self.min_origin.y, self.max_origin.y);
        self.visible_region.left =
            (self.origin.x - self.visible_span.x).max(self.world_region.left);
        self.visible_region.bottom =
            (self.origin.y - self.visible_span.y).max(self.world_region.bottom);
        self.visible_region.right =
            (self.origin.x + self.visible_span.x).min(self.world_region.right);
        self.visible_region.top = (self.origin.y + self.visible_span.y).min(self.world_region.top);
    }

    #[inline]
    pub fn check_pos_visible(&self, pos: &Vec2) -> bool {
        self.visible_region.covers(pos)
    }

    #[inline]
    pub fn contains(&self, pos: &Vec2) -> bool {
        self.world_region.covers(pos)
    }

    #[inline]
    pub fn max_collide_span(&self) -> f32 {
        self.max_collide_span
    }

    #[inline]
    pub fn player_pos(&self) -> Option<Vec2> {
        self.player_pos
    }

    #[inline]
    pub fn update_max_collide_span(&mut self, collide_span: f32) {
        if self.max_collide_span < collide_span {
            self.max_collide_span = collide_span;
        }
    }

    #[inline]
    pub fn update_player_pos(&mut self, player_pos: Option<Vec2>) {
        self.player_pos = player_pos;
    }

    #[inline]
    pub fn visible_region(&self) -> &RectRegion {
        &self.visible_region
    }
}
