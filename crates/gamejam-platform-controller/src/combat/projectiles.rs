use crate::combat::attackable::Attacked;
use crate::combat::combat_components::Invulnerable;
use crate::player_systems::player_components::Player;
use crate::scripting::scripted_game_entity::EntityScript;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::circle(8.)),
    GravityScale(|| GravityScale::from(0.)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    CollidingEntities,
)]
pub struct Projectile {
    pub collided: bool,
}

pub fn projectile_collision_system(
    mut commands: Commands,
    mut projectiles: Query<
        (
            Entity,
            &mut Projectile,
            &Transform,
            &CollidingEntities,
            Option<&mut EntityScript>,
        ),
        Without<Player>,
    >,
    player: Query<(Entity, Option<&Invulnerable>, &Transform), With<Player>>,
) {
    let Ok((player_entity, player_invulnerable, player_transform)) = player.get_single() else {
        return;
    };

    'outer: for (
        projectile_entity,
        mut projectile,
        transform,
        colliding_entities,
        entity_script,
    ) in projectiles.iter_mut()
    {
        if player_invulnerable.is_none() && !projectile.collided {
            for collides in colliding_entities.0.iter() {
                if *collides == player_entity {
                    projectile.collided = true;

                    commands.entity(player_entity).insert(Attacked {
                        damage: 10,
                        origin: transform.translation.xy(),
                        vector: player_transform.translation.xy() - transform.translation.xy(),
                        force: 4.0,
                    });
                }

                if let Some(mut s) = entity_script {
                    s.killed();
                } else {
                    commands.entity(projectile_entity).despawn();
                }

                continue 'outer;
            }
        }
    }
}
