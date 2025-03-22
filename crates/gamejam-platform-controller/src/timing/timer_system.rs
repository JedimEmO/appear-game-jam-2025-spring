use bevy::prelude::{Commands, Entity, Query, Res, Time};
use crate::scripting::scripted_game_entity::EntityScript;
use crate::timing::timing_component::TimerComponent;

pub fn timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimerComponent, Option<&mut EntityScript>)>
) {
    for (entity, mut timer, script) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if !timer.timer.finished() {
            continue;
        }

        commands.entity(entity).remove::<TimerComponent>();

        if let Some(mut script) = script {
            script.timer_callback(timer.timer_name);
        }
    }
}