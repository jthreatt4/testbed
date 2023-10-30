use crate::prelude::*;
use std::cmp;
use std::ops;

#[derive(Component, Copy, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn new_from2d(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(mut self, rhs: Position) -> Position {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Point> for Position {
    fn from(item: Point) -> Self {
        Position {
            x: item.x,
            y: item.y,
            z: 0,
        }
    }
}

impl From<(Point, i32)> for Position {
    fn from((point, new_z): (Point, i32)) -> Self {
        Position {
            x: point.x,
            y: point.y,
            z: new_z,
        }
    }
}

impl From<Position> for Point {
    fn from(item: Position) -> Self {
        Point {
            x: item.x,
            y: item.y,
        }
    }
}

// render utils
pub fn size_scaling(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&TileSize, &mut Transform)>,
) {
    if let Ok(primary) = primary_query.get_single() {
        for (sprite_size, mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                sprite_size.width / SCREEN_WIDTH as f32 * primary.width() as f32,
                sprite_size.height / SCREEN_HEIGHT as f32 * primary.height() as f32,
                1.0,
            );
            transform.scale = scale
        }
    }
}

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn position_translation(
    primary_query: Query<&Window>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    if let Ok(primary) = primary_query.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert_pos(pos.x as f32, primary.width() as f32, SCREEN_WIDTH as f32),
                convert_pos(
                    (pos.y + UI_HEIGHT / 2) as f32,
                    primary.height() as f32,
                    SCREEN_HEIGHT as f32,
                ),
                pos.z as f32,
            );
        }
    }
}