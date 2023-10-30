use crate::prelude::*;

#[derive(Component)]
struct ToolTipText;

#[derive(Component)]
struct ToolTipBox;

fn tooltip_ui(
    mut commands: Commands,
    font_manager: Res<FontManager>,
) {
    let game_log= GameLog::new();
    commands.insert_resource(game_log);

    commands
    // root node, just a black rectangle where the text will be
    .spawn((NodeBundle {
        // by default we set visible to false so it starts hidden
        visibility: Visibility::Hidden,
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
        ..Default::default()
    }, ToolTipBox))
    .with_children(|parent| {
        // text
        parent.spawn((TextBundle {
            visibility: Visibility::Hidden,
            style: Style {
                height: Val::Px(20. * 1.),
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            text: Text::from_section(
                "Goblin. HP: 2 / 2",
                TextStyle {
                    font: font_manager.font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        }, ToolTipText));
    });
}


// when leaving user input state, hide tooltip
fn hide_tooltip(
    mut text_box_query : ParamSet<(
        Query<&mut Visibility, With<ToolTipText>>,
        Query<&mut Visibility, With<ToolTipBox>>
    )>,
) {
    // update tooltip visibility
    for mut visible in text_box_query.p0().iter_mut() {
        *visible = Visibility::Hidden;
    }

    // update box visibility
    for mut visible in text_box_query.p1().iter_mut() {
        *visible = Visibility::Hidden;
    } 
}

// when user left clicks, update tooltip and make it visible
fn update_tooltip(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    // to get the mouse clicks
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Naming, &Position, Option<&Health>)>,
    // // query to get tooltip text and box
    mut text_box_query : ParamSet<(
        Query<(&mut Text, &mut Visibility), With<ToolTipText>>,
        Query<(&mut Style, &mut Visibility), With<ToolTipBox>>
    )>,
    // query to get the player field of view
    player_fov_q: Query<&FieldOfView, With<Player>>,
) {
    // if the user left clicks
    if buttons.just_pressed(MouseButton::Left) {
        // get the primary window
        let wnd = wnds.get_single().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {
            // get the size of the window
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single();

            let tile_size_x = wnd.width() / SCREEN_WIDTH as f32;
            let tile_size_y = wnd.height() / SCREEN_HEIGHT as f32;
            println!("tile_size [{}, {}]", tile_size_x, tile_size_y);

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0); 

            // transform world coordinates to our grid
            let grid_x = (pos_wld.x / tile_size_x) + (SCREEN_WIDTH / 2) as f32;
            let grid_y = (pos_wld.y / tile_size_y) + (SCREEN_HEIGHT / 2) as f32 + (UI_HEIGHT/2) as f32;
            let grid_position = Position{x: grid_x as i32, y: grid_y as i32, z:0};
            println!("world position {}", pos_wld);
            println!("grid position [{}, {}, {}]", grid_position.x, grid_position.y, grid_position.z);

            // now we go through all the entities with name to see which one is the nearest
            // some variables placeholders to save the entity name and its health
            let mut good_click = false;
            let mut s = String::new();
            let mut maxh = 0;
            let mut currenth = 0;
            // obtain also player fov
            let player_fov = player_fov_q.single();

            q_names.iter()
                .filter(|(name, pos, _)| {
                    // println!("world position {}", pos_wld);
                    // println!("grid position {}", grid_position);
                    println!("named position {} [{}, {}, {}]", name.0, pos.x, pos.y, pos.z);
                    **pos == grid_position && player_fov.visible_tiles.contains( &((**pos).into()) )
                })
                .for_each(|(name, _, health)| {
                    s = name.0.clone();
                    good_click = true;
                    // if it also has health component
                    if let Some(health) = health {
                        maxh = health.max;
                        currenth = health.current;
                    }
                });

            // update tooltip text
            for (mut text, mut visible) in text_box_query.p0().iter_mut() {
                if currenth > 0 {
                    text.sections[0].value = format!("{} HP: {} / {}", s, currenth, maxh);
                } else {
                    text.sections[0].value = format!("{}", s);
                }
                *visible = Visibility::Visible;
            }

            // update box position
            for (mut boxnode, mut visible) in text_box_query.p1().iter_mut() {
                if good_click {
                    boxnode.left = Val::Px(pos.x-100.0);
                    boxnode.bottom = Val::Px(pos.y);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
                
            }
        }
    }
}

pub struct TooltipsPlugin;
impl Plugin for TooltipsPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(OnExit(TurnState::StartScreen), tooltip_ui)
        .add_systems(Update, update_tooltip.run_if(in_state(TurnState::AwaitingInput)))        
        .add_systems(OnExit(TurnState::AwaitingInput), hide_tooltip);
    }
}