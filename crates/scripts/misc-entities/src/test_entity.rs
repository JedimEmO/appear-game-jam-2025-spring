use crate::gamejam::game::game_host::level_transition;
use game_entity_component::exports::gamejam::game::entity_resource::{EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings};
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{Collider, EventData, InsertableComponents, Interactable, despawn_entity, insert_components, play_animation, publish_event, remove_component, set_ticking, Direction};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = TestEntityScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct TestEntityScript {
    self_entity_id: u64,
    trigger_targets: Vec<u32>,
    activate_count: Cell<u32>,
}

impl TestEntityScript {
    fn new(params: StartupSettings) -> Self {
        play_animation("house_1", "idle", 1000, Direction::East, true);

        insert_components(&[InsertableComponents::Interactable(Interactable {
            message: "Hello, world".to_string(),
            range: 50.,
        })]);

        Self {
            self_entity_id: params.self_entity_id,
            trigger_targets: vec![],
            activate_count: Cell::new(0),
        }
    }
}

impl GuestGameEntity for TestEntityScript {
    fn tick(&self, _delta_t: f32) -> () {}

    fn interacted(&self) {
        level_transition(3, "entry")
    }

    fn attacked(&self) {}

    fn animation_finished(&self, animation_name: String) {
        let next_anim_name = match animation_name.as_str() {
            "idle" => "glowing",
            _ => "idle",
        };

        play_animation("house_1", next_anim_name, 1000, Direction::East, false);
    }

    fn receive_event(&self, _: Event) {}

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
