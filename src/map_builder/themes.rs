use crate::prelude::*;

pub struct Glyph {
    // the index in the atlas sprite sheet
    pub index: usize,
    // the color to tint the glyph
    pub color: Color,
    // the background color. If the glyph uses the fuel cell, not needed
    pub bkg_color: Option<Color>,
}

impl Glyph {
    fn new(index: usize, color: Color, bkg_color: Color) -> Self {
        Self {
            index,
            color,
            bkg_color: Some(bkg_color),
        }
    }
    fn new_nobkg(index: usize, color: Color) -> Self {
        Self {
            index,
            color,
            bkg_color: None,
        }
    }
}

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph> {
        let glyph_color = Color::rgba(0.3, 0.3, 0.3, 1.0);
        let wall_color = Color::rgba(0.05, 0.05, 0.05, 1.0);
        let floor_color = Color::rgba(0.529, 0.529, 0.529, 1.0);

        match tile_type {
            // index 219 is a full square
            TileType::Floor => Some(Glyph::new_nobkg(219, floor_color)),
            TileType::Wall => Some(Glyph::new('#' as usize, glyph_color, wall_color)),
            TileType::Exit => Some(Glyph::new('>' as usize, glyph_color, floor_color)),
            _ => None,
        }
    }
}
