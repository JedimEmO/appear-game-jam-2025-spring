use std::time::Duration;
use bevy::math::Vec2;
use bevy::prelude::{Component, Entity};
use bevy::time::{Timer, TimerMode};

#[derive(Component, Debug)]
pub struct ScheduledAttack {
    pub attacker: Entity,
    pub origin: Vec2,
    pub vector: Vec2,
    pub delay: Timer,
    pub force: f32,
    pub damage: u32,
}

#[derive(Component, Debug)]
pub struct Stamina {
    pub max_stamina: u32,
    pub current_stamina: u32,
    pub newly_consumed_stamina: u32,
    pub tick_timer: Timer
}


impl Stamina {
    pub fn default_player() -> Self {
        Self {
            max_stamina: 100,
            current_stamina: 100,
            newly_consumed_stamina: 0,
            tick_timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
        }
    }
}