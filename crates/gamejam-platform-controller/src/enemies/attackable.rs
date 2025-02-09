use bevy::prelude::*;
use crate::enemies::{Enemy, EnemyStateMachine, HitPoints};

/// An attackable entity (reacts to attacks)
#[derive(Component, Default)]
pub struct Attackable;

/// Attached when an attackable gets hit by an attack
#[derive(Component, Default)]
pub struct Attacked;

pub struct AttackablePlugin;

impl Plugin for AttackablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attackable_attacked_observer);
    }
}

pub fn attackable_attacked_observer(
    trigger: Trigger<OnAdd, Attacked>,
    time: Res<Time>,
    mut commands: Commands,
    mut attackables: Query<(Entity, &Attackable, Option<&mut Enemy>, Option<&mut HitPoints>)>
) {
    commands.entity(trigger.entity()).remove::<Attacked>();
    
    for (entity, attackable, mut enemy, mut hp) in attackables.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }
        
        if let Some(mut hp) = hp {
            hp.hp = hp.hp.saturating_sub(10);
        }
        
        if let Some(mut enemy) = enemy {
            enemy.state_machine = EnemyStateMachine::Staggered {
                staggered_at: time.elapsed_secs(),
                stagger_for: 0.5
            };
        }
    }
}