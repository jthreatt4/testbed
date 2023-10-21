use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Void,
}

pub struct Map {
    // for tiles like, wall, floor, ...
    pub tiles: Vec<TileType>,
    // entities occupying the tiles like, player, enemies, objects, ...
    pub occupation: Vec<Option<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            // TODO shouldn't this be divided by the tile size?
            tiles: vec![TileType::Floor; NUM_TILES],
            // TODO this is a weird instantiation
            occupation: vec![None; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<Position>>(&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0
            && position.x < SCREEN_WIDTH
            && position.y >= 0
            && position.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile<T: Into<Position>>(&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && self.tiles[map_idx(position.x, position.y)] == TileType::Floor
            || self.in_bounds(position)
                && self.tiles[map_idx(position.x, position.y)] == TileType::Exit
    }

    pub fn try_idx(&self, point: Position) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
