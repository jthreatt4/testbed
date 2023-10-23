use crate::prelude::*;

pub fn update_entities_visibility(
    player_fov_q: Query<&FieldOfView, With<Player>>,
    mut entities_q: Query<(
        Entity,
        &Position,
        &mut Visibility,
        Option<&MapTile>,
        Option<&mut Sprite>,
        Option<&mut TextureAtlasSprite>,
    )>,
) {
    let player_fov = player_fov_q.single();
    // iterate over entities with a position
    for (ent, pos, mut vis, map_tile, sprite, atlas_sprite) in entities_q.iter_mut() {
        // first check if it is a map tile or some other entity
        if let Some(_) = map_tile {
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                *vis = Visibility::Visible;
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(1.0);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(1.0);
                }
            } else if *vis == Visibility::Visible {
                *vis = Visibility::Visible;
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(0.1);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(0.1);
                }
            }
        } else {
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                // if it was not visible before, make it appear
                if *vis == Visibility::Hidden {
                    *vis = Visibility::Visible;
                }
            } else {
                *vis = Visibility::Hidden;
            }
        }
    }
}
