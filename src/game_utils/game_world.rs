use crate::game_utils::*;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Resource)]
pub struct GameWorld {
    cell_size: f32,
    world_width: f32,
    world_height: f32,
    world_region: RectRegion,
    min_origin: Vec2,
    max_origin: Vec2,
    origin: Vec2,
    visible_span: Vec2,
    visible_region: RectRegion,
    max_collide_span: f32,
    map: Vec<Vec<HashSet<Entity>>>,
}

impl GameWorld {
    pub fn new(
        cell_size: f32,
        row_count: usize,
        col_count: usize,
        window_width: f32,
        window_height: f32,
        visible_ext_size: f32,
        origin: &Vec2,
    ) -> Self {
        let world_width = col_count as f32 * cell_size;
        let world_height = row_count as f32 * cell_size;
        let mut game_world = Self {
            cell_size,
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
            map: vec![vec![HashSet::new(); col_count]; row_count],
        };

        game_world.set_origin(&origin);

        game_world
    }

    #[inline]
    pub fn row_count(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn col_count(&self) -> usize {
        self.map[0].len()
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

    pub fn add(&mut self, map_pos: &MapPos, entity: Entity) {
        self.map[map_pos.row][map_pos.col].insert(entity);
    }

    pub fn relocate(&mut self, entity: Entity, old_pos: &MapPos, new_pos: &MapPos) {
        self.map[old_pos.row][old_pos.col].remove(&entity);
        self.map[new_pos.row][new_pos.col].insert(entity);
    }

    #[inline]
    pub fn remove(&mut self, entity: &Entity, map_pos: &MapPos) {
        if !self.map[map_pos.row][map_pos.col].remove(entity) {
            error!(
                "Cannot remove entity {:?} from GameMap at position {:?}",
                entity, map_pos
            );
        }
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
    pub fn update_max_collide_span(&mut self, collide_span: f32) {
        if self.max_collide_span < collide_span {
            self.max_collide_span = collide_span;
        }
    }

    #[inline]
    pub fn visible_region(&self) -> &RectRegion {
        &self.visible_region
    }

    #[inline]
    pub fn get_map_pos(&self, pos: &Vec2) -> MapPos {
        MapPos {
            row: (pos.y / self.cell_size).floor() as usize,
            col: (pos.x / self.cell_size).floor() as usize,
        }
    }

    #[inline]
    pub fn get_row(&self, y: f32) -> usize {
        let i = (y / self.cell_size).floor() as i32;
        i.clamp(0, (self.row_count() - 1) as i32) as usize
    }

    #[inline]
    pub fn get_col(&self, x: f32) -> usize {
        let i = (x / self.cell_size).floor() as i32;
        i.clamp(0, (self.col_count() - 1) as i32) as usize
    }

    pub fn run_on_regions<F>(&self, regions: &Vec<MapRegion>, mut func: F) -> bool
    where
        F: FnMut(&Entity) -> bool,
    {
        for region in regions.iter() {
            if !self.run_on_region(region, &mut func) {
                return false;
            }
        }
        true
    }

    pub fn run_on_region<F>(&self, region: &MapRegion, mut func: F) -> bool
    where
        F: FnMut(&Entity) -> bool,
    {
        for row in region.start_row..=region.end_row {
            for col in region.start_col..=region.end_col {
                for entity in self.map[row][col].iter() {
                    if !func(entity) {
                        return false;
                    }
                }
            }
        }
        true
    }

    #[inline]
    pub fn get_region(&self, left: f32, bottom: f32, right: f32, top: f32) -> MapRegion {
        MapRegion {
            start_row: self.get_row(bottom),
            end_row: self.get_row(top),
            start_col: self.get_col(left),
            end_col: self.get_col(right),
        }
    }

    #[inline]
    pub fn get_region_from_rect(&self, rect_region: &RectRegion) -> MapRegion {
        self.get_region(
            rect_region.left,
            rect_region.bottom,
            rect_region.right,
            rect_region.top,
        )
    }
}
