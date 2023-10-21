use crate::prelude::*;
use bevy::app::AppExit;

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    player_position: Query<(Entity, &Position), With<Player>>,
    mut exit: EventWriter<AppExit>,
) {
    let (player_ent, pos) = player_position.single();
    let mut action = true;
    let mut wait = false;

    let mut new_position = pos.clone();

    let key = keyboard_input.get_pressed().next().clone();

    if let Some(key) = key {
        println!("key pressed: {:?}", key);

        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Up => new_position.y += 1,
            KeyCode::Down => new_position.y -= 1,
            // TODO pickup item at position
            // TODO show inventory
            // TODO equip item
            KeyCode::Escape => {
                exit.send(AppExit);
            }
            _ => wait = true,
        }

        // move to new position
        if new_position != *pos {
            println!("moving player");
            commands.spawn(WantsToMove {
                entity: player_ent,
                destination: new_position,
            });
        } else if wait {
            println!("waiting for input")
        }

        if action {
            println!("switch to game world turn")
        }

        keyboard_input.reset_all()
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input);
    }
}
