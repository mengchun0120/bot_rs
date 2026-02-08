use crate::game_utils::*;
use bevy::prelude::*;
use std::collections::{HashSet, hash_set::Iter};

#[derive(Resource)]
pub struct GameMap {
    cell_size: f32,
    pub map: Vec<Vec<HashSet<Entity>>>,
}

pub struct MapIterator<'a> {
    game_map: &'a GameMap,
    row: usize,
    col: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
    iter: Iter<'a, Entity>,
}

impl GameMap {
    pub fn new(row_count: usize, col_count: usize, cell_size: f32) -> Self {
        Self {
            cell_size,
            map: vec![vec![HashSet::new(); col_count]; row_count],
        }
    }

    pub fn add(&mut self, map_pos: &MapPos, entity: Entity) {
        self.map[map_pos.row][map_pos.col].insert(entity);
    }

    pub fn relocate(&mut self, entity: Entity, old_pos: &MapPos, new_pos: &MapPos) {
        self.map[old_pos.row][old_pos.col].remove(&entity);
        self.map[new_pos.row][new_pos.col].insert(entity);
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
    pub fn remove(&mut self, entity: &Entity, map_pos: &MapPos) {
        if !self.map[map_pos.row][map_pos.col].remove(entity) {
            error!(
                "Cannot remove entity {:?} from GameMap at position {:?}",
                entity, map_pos
            );
        }
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

    pub fn map_iter<'a>(&'a self, region: &MapRegion) -> MapIterator<'a> {
        MapIterator::new(&self, region)
    }
}

impl<'a> MapIterator<'a> 
{
    pub fn new(game_map: &'a GameMap, region: &MapRegion) -> Self {
        Self {
            game_map,
            row: region.start_row,
            col: region.start_col,
            end_row: region.end_row,
            start_col: region.start_col,
            end_col: region.end_col,
            iter: game_map.map[region.start_row][region.start_col].iter(),
        }
    }

    fn move_to_next_cell(&mut self) -> bool {
        if self.col < self.end_col {
            self.col += 1;
            self.iter = self.game_map.map[self.row][self.col].iter();
            true
        } else if self.row < self.end_row {
            self.col = self.start_col;
            self.row += 1;
            self.iter = self.game_map.map[self.row][self.col].iter();
            true
        } else {
            false
        }
    }

}

impl<'a> Iterator for MapIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.iter.next() {
            Some(*entity)
        } else {
            while self.move_to_next_cell() {
                if let Some(entity) = self.iter.next() {
                    return Some(*entity);
                }
            }
            None
        }
    }
}