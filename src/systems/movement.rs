use crate::prelude::*;

// spawns movement components to move entities
pub fn movement(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    move_messages: Query<(Entity, &WantsToMove)>,
    // what is a mover?
    // mut movers: Query<(Entity, &mut Position, &mut FieldOfView)>,
    mut movers: Query<(Entity, &mut Position)>,
) {
    for (message_ent, move_signal) in move_messages.iter() {
        // if the movement is valid
        if mb.map.can_enter_tile(move_signal.destination) {
            // if mb.map.is_tile_occupied(move_signal.destination) {
                println!("player can enter");

            // get the entity and its alive status
            // if let Ok((move_ent, mut position, mut fov)) = movers.get_mut(move_signal.entity) {
            if let Ok((move_ent, mut position)) = movers.get_mut(move_signal.entity) {
                // update occcupation map
                println!("updating player");
                mb.move_entity_occupation(move_ent, *position, move_signal.destination);
                // and execute the movement
                position.x = move_signal.destination.x;
                position.y = move_signal.destination.y;
                // mark the fov to be updated
                // fov.is_dirty = true;
            }

            // }
        }
        // delete the move message
        commands.entity(message_ent).despawn();
    }
}
