use crate::prelude::*;

mod camera;
mod movement;
mod player_input;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player_input::PlayerInputPlugin);
            // .add_systems(Update, (camera::camera_move));
    }
}

// TODO move this somewhere more appropriate
pub fn spawn_player(mut commands: Commands, atlas: Res<CharsetAsset>, mut mb: ResMut<MapBuilder>) {
    println!("try player");

    let player_start = mb.player_start;

    let entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: atlas.atlas.clone(),
                sprite: TextureAtlasSprite {
                    // TODO this should probably use the tile size
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    index: '@' as usize,
                    ..Default::default()
                },
                ..Default::default()
            },
            Player,
            Position {
                x: player_start.x,
                y: player_start.y,
                z: 2,
            },
            TileSize::square(1.0),
            // Health{ current: 10, max: 20},
            // FieldOfView::new(8),
            // Damage(1),
        ))
        .id();

    mb.entity_occupy_tile(entity, player_start);
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, (movement::movement, /*camera::camera_move*/));
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(AwaitingInputPlugin);
    }
}
