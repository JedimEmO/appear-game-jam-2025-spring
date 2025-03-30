use crate::player_const_rules::MAX_SPEED;
use bevy::math::Vec2;
use bevy::prelude::{Component, Entity, Event};

#[derive(Component, Debug, Default)]
pub enum FacingDirection {
    West,
    #[default]
    East,
}

impl FacingDirection {
    pub fn to_bool(&self) -> bool {
        match self {
            Self::West => true,
            _ => false,
        }
    }
}

#[derive(Event)]
pub struct EntityInput {
    pub entity: Entity,
    pub input: Input,
}

pub enum Input {
    Move(Vec2),
    Jump,
}

#[derive(Component)]
pub struct MovementData {
    pub max_speed: f32,
    pub feet_height: f32,
}

impl MovementData {
    pub fn default_enemy() -> Self {
        Self {
            max_speed: 100.0,
            feet_height: 16.0,
        }
    }

    pub fn default_player() -> Self {
        Self {
            max_speed: MAX_SPEED,
            feet_height: 16.,
        }
    }
}
