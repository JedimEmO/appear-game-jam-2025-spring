use crate::scripting::scripted_game_entity::EntityScript;
use crate::timing::timing_component::{TimerComponent, TimerData};
use bevy::prelude::{Bundle, Commands, Component, Entity, EntityCommands, Query, Res, Time, Timer, TimerMode};
use std::time::Duration;

pub fn timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimerComponent, Option<&mut EntityScript>)>,
) {
    for (entity, mut timer, mut script) in query.iter_mut() {
        for timer in timer.timers.iter_mut() {
            timer.timer.tick(time.delta());

            if !timer.timer.finished() {
                continue;
            }

            if let Some(script) = script.as_deref_mut() {
                script.timer_callback(timer.timer_name);
            }

            if let Some(f) = timer.on_expiration.take() {
                f(&mut commands.entity(entity));
            }
        }

        timer.timers.retain(|v| !v.timer.finished());

        if timer.timers.is_empty() {
            commands.entity(entity).remove::<TimerComponent>();
        }
    }
}

pub fn add_timer_to_entity(
    commands: &mut EntityCommands,
    timer: Option<&mut TimerComponent>,
    data: TimerData,
) {
    if let Some(timer) = timer {
        timer.timers.push(data);
    } else {
        commands.insert(TimerComponent { timers: vec![data] });
    }
}
pub fn add_timed_component_to_entity<T: Bundle>(
    commands: &mut EntityCommands,
    timer: Option<&mut TimerComponent>,
    component: T,
    duration: Duration,
) {
    commands.insert(component);

    add_timer_to_entity(
        commands,
        timer,
        TimerData {
            timer_name: 0,
            timer: Timer::new(duration, TimerMode::Once),
            on_expiration: Some(Box::new(|cmds| {
                cmds.remove::<T>();
            })),
        },
    );
}
