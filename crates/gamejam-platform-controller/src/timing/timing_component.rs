use bevy::prelude::{Commands, Component, EntityCommands};
use bevy::time::Timer;

#[derive(Component)]
pub struct TimerComponent {
    pub timers: Vec<TimerData>,
}

pub struct TimerData {
    pub timer_name: u32,
    pub timer: Timer,
    pub on_expiration: Option<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}
