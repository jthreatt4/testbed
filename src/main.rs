mod components;
mod map_builder;
mod resources;
mod systems;
mod ui;
mod utils;

mod prelude {
    // bevy base modules
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;
    pub use bevy::winit::WinitSettings;

    // bracket-lib base library
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub const UI_HEIGHT: i32 = 10;

    pub use crate::components::*;
    pub use crate::map_builder::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
    pub use crate::ui::*;
    pub use crate::utils::*;
}

use prelude::*;

// #[derive(Component)]
// struct Position { x: f32, y:f32 }

// fn print_position_systems(query: Query<&Position>) {
//     for position in &query {
//         println!("position: {} {}", position.x, position.y)
//     }
// }

// struct Entity(u64);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    turn_state: Res<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if *turn_state.get() == TurnState::LoadAssets {
        println!("try assets");
        // Setup the sprite sheet
        let texture_handle = asset_server.load("terminal8x8_transparent.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // add sprite atlas as resource
        commands.insert_resource(CharsetAsset {
            atlas: texture_atlas_handle.clone(),
        });

        let mut cam = Camera2dBundle::default();
        cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
        commands.spawn((MainCamera, cam));

        // update turn state
        next_state.set(TurnState::StartScreen)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roguelike TestBed".to_string(),
                        resolution: (SCREEN_WIDTH as f32 * 10.0, SCREEN_HEIGHT as f32 * 10.0)
                            .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_state::<TurnState>()
        .add_systems(Startup, setup)
        .add_plugins(MapPlugin)
        .add_plugins(SystemsPlugin)
        .add_plugins(UIPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run()
}
