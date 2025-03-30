use crate::movement_systems::movement_components::FacingDirection;
use bevy::prelude::{Changed, Query, Sprite};

pub fn movement_direction_system(
    mut query: Query<(&mut Sprite, &FacingDirection), Changed<FacingDirection>>,
) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = direction.to_bool();
    }
}
