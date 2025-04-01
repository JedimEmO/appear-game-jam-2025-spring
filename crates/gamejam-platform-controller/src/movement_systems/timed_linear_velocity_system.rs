use crate::movement_systems::movement_components::{ApplyTimedLinearVelocity, IgnoreDampening};
use avian2d::prelude::LinearVelocity;
use bevy::log::info;
use bevy::prelude::{Commands, Entity, Query, Res, Time};

pub fn timed_linear_velocity_system(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, &mut LinearVelocity, &mut ApplyTimedLinearVelocity)>,
) {
    for (entity, mut linvel, mut vel) in entities.iter_mut() {
        vel.timer.tick(time.delta());

        if vel.timer.finished() {
            commands.entity(entity).remove::<IgnoreDampening>();
            commands.entity(entity).remove::<ApplyTimedLinearVelocity>();

            continue;
        } else {
            commands.entity(entity).insert(IgnoreDampening);
        }

        let remaining = vel.timer.remaining().as_secs_f32();
        let velocity = (vel.acceleration_function)(remaining) * time.delta_secs();

        linvel.x += velocity.x;
        linvel.y += velocity.y;
    }
}
