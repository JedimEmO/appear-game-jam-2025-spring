use crate::combat::combat_components::{Health, Invulnerable};
use crate::movement_systems::movement_components::ApplyTimedLinearVelocity;
use crate::scripting::scripted_game_entity::EntityScript;
use bevy::math::vec2;
use bevy::prelude::*;
use std::time::Duration;

/// An attackable entity (reacts to attacks)
#[derive(Component, Default, Reflect)]
pub struct Attackable;

/// Attached when an attackable gets hit by an attack
#[derive(Component, Default, Reflect)]
pub struct Attacked {
    pub damage: u32,
    pub origin: Vec2,
    pub vector: Vec2,
    pub force: f32,
}

pub struct AttackablePlugin;

impl Plugin for AttackablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attackable_attacked_observer);
    }
}

pub fn attackable_attacked_observer(
    trigger: Trigger<OnAdd, Attacked>,
    mut commands: Commands,
    mut attackables: Query<
        (
            Entity,
            &Attackable,
            &Attacked,
            Option<&mut Health>,
            Option<&mut EntityScript>,
        ),
        Without<Invulnerable>,
    >,
) {
    commands.entity(trigger.entity()).remove::<Attacked>();

    for (entity, _attackable, attack, hp, script) in attackables.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }

        if let Some(mut hp) = hp {
            hp.0.consume(attack.damage);
        }

        let pushback_time = 0.1 + attack.force * 0.03;
        let push_direction = attack.vector;

        commands.entity(entity).insert(ApplyTimedLinearVelocity {
            timer: Timer::new(Duration::from_secs_f32(pushback_time), TimerMode::Once),
            acceleration_function: Box::new(move |remaining_time: f32| {
                let dir = vec2(push_direction.normalize().x, 1.2).normalize()
                    * (remaining_time / pushback_time)
                    * 3000.;
                dir
            }),
        });

        if let Some(mut script) = script {
            script.attacked()
        }
    }
}
