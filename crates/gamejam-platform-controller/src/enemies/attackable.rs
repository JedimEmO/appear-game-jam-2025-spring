use crate::enemies::HitPoints;
use crate::scripting::scripted_game_entity::EntityScript;
use bevy::prelude::*;

/// An attackable entity (reacts to attacks)
#[derive(Component, Default, Reflect)]
pub struct Attackable;

/// Attached when an attackable gets hit by an attack
#[derive(Component, Default, Reflect)]
pub struct Attacked {
    pub damage: u32,
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
    mut attackables: Query<(
        Entity,
        &Attackable,
        &Attacked,
        Option<&mut HitPoints>,
        Option<&mut EntityScript>,
    )>,
) {
    commands.entity(trigger.entity()).remove::<Attacked>();

    for (entity, _attackable, attack, hp, script) in attackables.iter_mut() {
        if entity != trigger.entity() {
            continue;
        }

        if let Some(mut hp) = hp {
            hp.hp = hp.hp.saturating_sub(attack.damage);
        }

        if let Some(mut script) = script {
            script.attacked()
        }
    }
}
