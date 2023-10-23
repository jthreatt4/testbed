use crate::prelude::*;
use template::Templates;

mod camera;
mod fov;
mod movement;
mod player_input;
mod template;
mod update_entities_visibility;

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
            Player {map_level: 0},
            Naming("Player".to_string()),
            Position {
                x: player_start.x,
                y: player_start.y,
                z: 2,
            },
            TileSize::square(1.0),
            Health {
                current: 10,
                max: 20,
            },
            FieldOfView::new(8),
            Damage(1),
        ))
        .id();

    mb.entity_occupy_tile(entity, player_start);
}

pub fn spawn_level(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<MapBuilder>,
    player_q: Query<&Player>,
) {
    // start by getting the player, if it exists, to get the level
    // if it doesnt exist, then it is level 0
    let mut level = 0;
    if player_q.iter().count() > 0 {
        level = player_q.single().map_level;
    }

    // load template from file and spawn entities
    let template = Templates::load();
    template.spawn_entities(&mut commands, atlas, level as usize, &mut mb);
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(TurnState::StartScreen), (spawn_player, spawn_level))
            .add_systems(
                Update,
                (
                    movement::movement,
                    fov::fov,
                    update_entities_visibility::update_entities_visibility,
                    camera::camera_move,
                )
                    .run_if(in_state(TurnState::AwaitingInput)),
            );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(AwaitingInputPlugin);
    }
}
