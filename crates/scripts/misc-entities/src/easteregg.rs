use crate::gamejam::game::game_host::win;
use crate::gamejam::game::game_host::play_sound_once;
use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    Direction, InsertableComponents, Interactable,
    insert_components, play_animation
};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
use script_utils::script_parameters::ScriptParams;

export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = EasterEggScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct EasterEggScript {
    _self_entity_id: u64,
}

impl EasterEggScript {
    fn new(settings: StartupSettings) -> Self {
        play_animation("easteregg", "idle", 400, Direction::East, true);

        insert_components(&[InsertableComponents::Interactable(Interactable {
            message: "<up> Smash the last easter egg and kill off the easter bunny species!".to_string(),
            range: 50.,
        })]);

        Self {
            _self_entity_id: settings.self_entity_id
        }
    }
}

impl GuestGameEntity for EasterEggScript {
    fn tick(&self, _delta_t: f32) -> () {}

    fn interacted(&self) {
        win();
    }

    fn attacked(&self) {}

    fn animation_finished(&self, animation_name: String) {}

    fn receive_event(&self, _: Event) {}

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
