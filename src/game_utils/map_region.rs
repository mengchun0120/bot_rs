use crate::game_utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct MapRegion {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl MapRegion {
    #[inline]
    pub fn contains(&self, pos: &MapPos) -> bool {
        pos.row >= self.start_row
            && pos.row <= self.end_row
            && pos.col >= self.start_col
            && pos.col <= self.end_col
    }

    pub fn sub(&self, other: &MapRegion) -> Vec<MapRegion> {
        let mut result = Vec::new();

        if self.start_row > other.end_row
            || self.end_row < other.start_row
            || self.start_col > other.end_col
            || self.end_col < other.start_col
        {
            result.push(self.clone());
            return result;
        }

        let mut start_row = self.start_row;
        let mut end_row = self.end_row;

        if self.start_row < other.start_row {
            result.push(MapRegion {
                start_row: self.start_row,
                end_row: other.start_row - 1,
                start_col: self.start_col,
                end_col: self.end_col,
            });
            start_row = other.start_row;
        }

        if self.end_row > other.end_row {
            result.push(MapRegion {
                start_row: other.end_row + 1,
                end_row: self.end_row,
                start_col: self.start_col,
                end_col: self.end_col,
            });
            end_row = other.end_row;
        }

        if self.start_col < other.start_col {
            result.push(MapRegion {
                start_row,
                end_row,
                start_col: self.start_col,
                end_col: other.start_col - 1,
            });
        }

        if self.end_col > other.end_col {
            result.push(MapRegion {
                start_row,
                end_row,
                start_col: other.end_col + 1,
                end_col: self.end_col,
            });
        }

        result
    }

    pub fn intersect(&self, other: &MapRegion) -> Vec<MapRegion> {
        let mut result = Vec::new();

        if self.start_row > other.end_row
            || self.end_row < other.start_row
            || self.start_col > other.end_col
            || self.end_col < other.start_col
        {
            return result;
        }

        result.push(MapRegion {
            start_row: self.start_row.max(other.start_row),
            end_row: self.end_row.min(other.end_row),
            start_col: self.start_col.max(other.start_col),
            end_col: self.end_col.min(other.end_col),
        });

        result
    }
}
