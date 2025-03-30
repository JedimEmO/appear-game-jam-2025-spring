use crate::enemies::attackable::{Attackable, Attacked};
use crate::enemies::combat_components::ScheduledAttack;
use avian2d::prelude::Collider;
use bevy::prelude::{Commands, Entity, Query, Res, Transform, With};
use bevy::time::Time;

pub fn scheduled_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut attacks: Query<(Entity, &mut ScheduledAttack)>,
    attackables: Query<(Entity, &Transform, &Collider), With<Attackable>>,
) {
    let delta = time.delta();

    for (attack_entity, mut attack) in attacks.iter_mut() {
        attack.delay.tick(delta);

        if !attack.delay.finished() {
            continue;
        }

        for (attackable_entity, attackable_transform, collider) in attackables.iter() {
            if attackable_entity == attack.attacker {
                continue;
            }

            let distance = attack.vector.length();

            if collider.intersects_ray(
                attackable_transform.translation.truncate(),
                0.,
                attack.origin,
                attack.vector.normalize(),
                distance,
            ) {
                commands.entity(attackable_entity).insert(Attacked {
                    damage: attack.damage,
                });
            }
        }

        commands.entity(attack_entity).despawn();
    }
}
