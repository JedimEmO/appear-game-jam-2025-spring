use bevy::prelude::*;


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
    mut commands: Commands,
    attackables: Query<(Entity, &Attackable)>
) {
    commands.entity(trigger.entity()).remove::<Attacked>();
}