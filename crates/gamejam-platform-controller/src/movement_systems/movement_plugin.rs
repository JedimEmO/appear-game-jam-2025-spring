use crate::movement_systems::direction_system::movement_direction_system;
use crate::movement_systems::grounded_system::grounded_system;
use crate::movement_systems::movement_components::EntityInput;
use crate::movement_systems::movement_dampening_system::movement_dampening_system;
use crate::movement_systems::movement_system::{movement_system, rolling_removed_observer};
use crate::movement_systems::timed_linear_velocity_system::timed_linear_velocity_system;
use crate::scripting::scripted_game_entity::scripted_entity_uniform_system;
use crate::GameStates;
use bevy::app::{App, FixedUpdate};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityInput>()
            .add_systems(
                FixedUpdate,
                (
                    (
                        scripted_entity_uniform_system,
                        grounded_system,
                        movement_system,
                        movement_direction_system,
                        timed_linear_velocity_system,
                    ),
                    movement_dampening_system,
                )
                    .chain()
                    .run_if(in_state(GameStates::GameLoop)),
            )
            .add_observer(rolling_removed_observer);
    }
}
