use bevy::prelude::Component;
use bevy::time::Timer;

#[derive(Component, Debug)]
pub struct TimerComponent {
    pub timer_name: u32,
    pub timer: Timer
}