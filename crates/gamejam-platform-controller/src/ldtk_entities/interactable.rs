use bevy::prelude::*;
use crate::player_components::Player;
use crate::ui::interactable_hint::{make_interactable_hint, InteractableHintComponent};

#[derive(Component)]
pub struct Interactable {
    pub action_hint: String
}

#[derive(Component)]
pub struct InteractableInRange;

#[derive(Component)]
pub struct Interacted;

pub fn interactable_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, (With<Player>, Without<InteractableHintComponent>)>,
    interactables_query: Query<(Entity, &Transform, &Interactable), Without<InteractableHintComponent>>,
    mut hint_component: Query<(Entity, &mut Text), With<InteractableHintComponent>>
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let mut set = false;

    for (entity, entity_transform, interactable) in interactables_query.iter() {
        let mut entity_commands = commands.entity(entity);

        if entity_transform.translation.distance(player.translation) > 20. {
            entity_commands.remove::<InteractableInRange>();
            continue;
        }

        entity_commands.insert(InteractableInRange);

        if let Ok((_entity, mut text)) = hint_component.get_single_mut() {
            text.0 = interactable.action_hint.clone();
        } else {
            commands.spawn(make_interactable_hint(&asset_server, interactable.action_hint.clone()));
        }

        set = true;
        break;
    }

    if !set {
        if let Ok((_entity, mut text)) = hint_component.get_single_mut() {
            text.0.clear();
        }
    }
}