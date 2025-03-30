use bevy::math::Vec2;
use bevy::prelude::{Component, Entity};
use bevy::time::Timer;

#[derive(Component, Debug)]
pub struct ScheduledAttack {
    pub attacker: Entity,
    pub origin: Vec2,
    pub vector: Vec2,
    pub delay: Timer,
    pub force: f32,
    pub damage: u32,
}
