use crate::enemies::{Enemy, EnemyStateMachine, HitPoints};
use crate::graphics::sprite_collection::SpriteCollection;
use crate::scripting::scripted_game_entity::{EntityScript, ScriptEvent};
use bevy::prelude::*;

/// An attackable entity (reacts to attacks)
#[derive(Component, Default, Reflect)]
pub struct Attackable;

/// Attached when an attackable gets hit by an attack
#[derive(Component, Default, Reflect)]
pub struct Attacked;

pub struct AttackablePlugin;

impl Plugin for AttackablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attackable_attacked_observer);
    }
}

pub fn attackable_attacked_observer(
    trigger: Trigger<OnAdd, Attacked>,
    mut event_writer: EventWriter<ScriptEvent>,
    time: Res<Time>,
    sprites: Res<SpriteCollection>,
    mut commands: Commands,
    mut attackables: Query<(
        Entity,
        &Attackable,
        Option<&mut Enemy>,
        Option<&mut HitPoints>,
        Option<&mut EntityScript>,
    )>,
) {
    commands.entity(trigger.entity()).remove::<Attacked>();

    for (entity, _attackable, enemy, hp, script) in attackables.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }

        if let Some(mut hp) = hp {
            hp.hp = hp.hp.saturating_sub(10);
        }

        if let Some(mut enemy) = enemy {
            enemy.state_machine = EnemyStateMachine::Staggered {
                staggered_at: time.elapsed_secs(),
                stagger_for: 0.5,
            };
        }

        if let Some(mut script) = script {
            script.attacked(&mut commands, &sprites, &mut event_writer)
        }
    }
}
