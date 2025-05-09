use bevy::math::Vec2;
use bevy::prelude::{Component, Entity, Resource};
use bevy::time::{Timer, TimerMode};
use std::time::Duration;
use haalka::prelude::Mutable;
use crate::player_systems::player_components::StatBarMutables;

#[derive(Component, Debug)]
pub struct ScheduledAttack {
    pub attacker: Entity,
    pub origin: Vec2,
    pub vector: Vec2,
    pub delay: Timer,
    pub force: f32,
    pub damage: u32,
}

#[derive(Component)]
pub struct Invulnerable;

#[derive(Component)]
pub struct Boss;

#[derive(Resource, Default)]
pub struct BossHealth(pub StatBarMutables, pub Mutable<bool>);

#[derive(Debug)]
pub struct Stat {
    pub max: u32,
    pub current: u32,
    pub newly_consumed: u32,
    pub tick_timer: Timer,
    pub regenerate: Option<u32>,
}

impl Stat {
    pub fn tick(&mut self, delta: Duration) {
        self.tick_timer.tick(delta);

        self.newly_consumed = self
            .newly_consumed
            .saturating_sub((self.max as f32 * 0.9 * delta.as_secs_f32()) as u32);

        if self.tick_timer.just_finished() {
            if let Some(regen) = self.regenerate {
                if self.current < self.max {
                    self.current = self.max.min(self.current + regen);
                }
            }
        }
    }

    pub fn consume(&mut self, amount: u32) {
        if self.current < amount {
            self.newly_consumed = self.current;
            self.current = 0;

            return;
        }

        self.current -= amount;
        self.newly_consumed = amount;
    }
    pub fn try_consume(&mut self, amount: u32) -> bool {
        if self.current < amount {
            return false;
        }

        self.current -= amount;
        self.newly_consumed = amount;

        true
    }
}

#[derive(Component, Debug)]
pub struct Stamina(pub Stat);

#[derive(Component, Debug)]
pub struct Health(pub Stat);

impl Stamina {
    pub fn default_player() -> Self {
        Self(Stat {
            max: 100,
            current: 100,
            newly_consumed: 0,
            tick_timer: Timer::new(Duration::from_millis(75), TimerMode::Repeating),
            regenerate: Some(1),
        })
    }

    pub fn new(value: u32) -> Self {
        Self(Stat {
            max: value,
            current: value,
            newly_consumed: 0,
            tick_timer: Timer::new(Duration::from_millis(75), TimerMode::Repeating),
            regenerate: Some(1),
        })
    }
}

impl Health {
    pub fn default_player() -> Self {
        Self(Stat {
            max: 100,
            current: 100,
            newly_consumed: 0,
            tick_timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            regenerate: None,
        })
    }

    pub fn new(value: u32) -> Self {
        Self(Stat {
            max: value,
            current: value,
            newly_consumed: 0,
            tick_timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            regenerate: None,
        })
    }
}
