use crate::gamejam::game::game_host::set_bonfire;
use crate::gamejam::game::game_host::level_transition;
use crate::gamejam::game::game_host::play_sound_once;
use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    Collider, Direction, EventData, InsertableComponents, Interactable, despawn_entity,
    insert_components, play_animation, publish_event, remove_component, set_ticking,
};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
use script_utils::script_parameters::ScriptParams;

export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = BonfireScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct BonfireScript {
    _self_entity_id: u64,
    level_index: u32,
    spawn_name: String,
}

impl BonfireScript {
    fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);

        play_animation("bonfire", "idle", 400, Direction::East, true);

        insert_components(&[InsertableComponents::Interactable(Interactable {
            message: "<up> Rest at bonfire".to_string(),
            range: 50.,
        })]);

        Self {
            _self_entity_id: settings.self_entity_id,
            level_index: params.get_parameter::<u32>("level-index").unwrap(),
            spawn_name: params.get_parameter::<String>("spawn-name").unwrap(),
        }
    }
}

impl GuestGameEntity for BonfireScript {
    fn tick(&self, _delta_t: f32) -> () {}

    fn interacted(&self) {
        play_sound_once("audio/rest.wav");
        set_bonfire(self.level_index, &self.spawn_name);
    }

    fn attacked(&self) {}

    fn animation_finished(&self, animation_name: String) {}

    fn receive_event(&self, _: Event) {}

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
