use crate::prelude::*;
use bracket_lib::prelude::Rect;

mod map;
pub use map::*;

const NUM_ROOMS: usize = 5;

#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    walls: Vec<Rect>,
    rooms: Vec<Rect>,
    pub player_start: Position,
    // pub enemies_start: Vec<Position>,
    // pub amulet_start: Position,
    // pub theme:Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            walls: Vec::new(),
            rooms: Vec::new(),
            player_start: Position { x: 0, y: 0, z: 0 },
        };
        mb
    }

    fn fill() {}

    fn buid_random_rooms() {}

    fn apply_horizontal_tunnel_walls() {}

    fn apply_vertical_tunnel_walls() {}

    fn build_corridors() {}

    pub fn entity_occupy_tile(&mut self, entity: Entity, position: Position) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = Some(entity);
    }

    pub fn free_occupy_tile(&mut self, position: Position) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = None;
    }

    pub fn move_entity_occupation(&mut self, entity: Entity, old_p: Position, new_p: Position) {
        let old_idx = map_idx(old_p.x, old_p.y);
        let new_idx = map_idx(new_p.x, new_p.y);
        self.map.occupation[old_idx] = None;
        self.map.occupation[new_idx] = Some(entity);
    }
}

pub fn build_map(mut commands: Commands, player_q: Query<&Player>) {
    // create map
    let mut mb = MapBuilder::new();

    commands.insert_resource(mb);
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_map);
    }
}
