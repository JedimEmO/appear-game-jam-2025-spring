use crate::ldtk_entities::player_collidable_entity::PlayerCollidableInRangeForCheck;
use crate::ldtk_entities::player_spawn::RequestedPlayerSpawn;
use crate::player_systems::player_components::Player;
use avian2d::collision::Collider;
use avian2d::math::Vector;
use avian2d::prelude::{AnyCollider, Rotation};
use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LevelSelection};
use crate::GameStates;

#[derive(Component, Default)]
pub struct LevelTransition {
    pub target_level_index: i32,
    pub target_player_spawn_name: String,
}

pub fn level_transition_system(
    mut commands: Commands,
    mut level_select: ResMut<LevelSelection>,
    mut next_state: ResMut<NextState<GameStates>>,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    player_collidable_query: Query<
        (&Transform, &LevelTransition, &Collider),
        With<PlayerCollidableInRangeForCheck>,
    >,
) {
    let Ok((player_entity, player_transform, player_collider)) = player_query.get_single() else {
        return;
    };

    for (transform, transition, collider) in player_collidable_query.iter() {
        if player_collider
            .contact_manifolds(
                collider,
                Vector::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                ),
                Rotation::default(),
                Vector::new(transform.translation.x, transform.translation.y),
                Rotation::default(),
                10.,
            )
            .len()
            > 0
        {
            info!("Switching level");
            commands.entity(player_entity).insert(RequestedPlayerSpawn {
                spawn_name: transition.target_player_spawn_name.clone(),
            });

            *level_select = LevelSelection::index(transition.target_level_index as usize);
            next_state.set(GameStates::LoadLevel);
        }
    }
}

pub fn spawn_level_transition_observer(
    _trigger: Trigger<OnAdd, LevelTransition>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &EntityInstance, &LevelTransition),
        Added<LevelTransition>,
    >,
) {
    for (entity, _transform, entity_instance, _transition) in query.iter_mut() {
        let mut entity = commands.entity(entity);

        entity.insert(Collider::rectangle(
            entity_instance.width as f32,
            entity_instance.height as f32,
        ));
    }
}
