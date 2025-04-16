use crate::gamejam::game::game_host::play_sound_once;
use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{Collider, EventData, InsertableComponents, get_game_data_kv_int, insert_components, play_animation, remove_component, set_game_data_kv_int, Direction, Interactable};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = RubbleScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct RubbleScript {
    _self_entity_id: u64,
    sprite_name: String,
    death_animation_duration: u32,
    invulnerable: bool,
    death_sound: Option<String>
}

impl RubbleScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let sprite_name = params.get_parameter::<String>("sprite-name").unwrap();

        if let Some(message) = params.get_parameter::<String>("message") {
            insert_components(&[InsertableComponents::Interactable(Interactable {
                message,
                range: 30.,
            })]);
        }

        let death_sound = params.get_parameter::<String>("death-sound");
        let death_animation_duration = params.get_parameter::<u32>("death-duration").unwrap_or(400);
        let physical = params.get_parameter::<bool>("physical").unwrap_or(false);
        let collider = params
            .get_list_parameter::<f32>("collider-size")
            .unwrap_or(vec![32., 32.]);
        let invulnerable = params
            .get_parameter::<bool>("invulnerable")
            .unwrap_or(false);

        if !invulnerable {
            insert_components(&[
                InsertableComponents::Attackable,
                InsertableComponents::Collider(Collider {
                    width: collider[0],
                    height: collider[1],
                    physical,
                }),
            ]);
        }

        play_animation(&sprite_name, "idle", 1000, Direction::East, true);

        Self {
            _self_entity_id: self_entity_id,
            sprite_name,
            death_animation_duration,
            invulnerable,
            death_sound
        }
    }
}

impl GuestGameEntity for RubbleScript {
    fn tick(&self, delta_t: f32) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {
        if self.invulnerable {
            return;
        }

        play_animation(
            &self.sprite_name,
            "death",
            self.death_animation_duration,
            Direction::East,
            false,
        );
        
        if let Some(sound) = &self.death_sound {
            play_sound_once(sound);
        }
        
        remove_component("avian2d::dynamics::rigid_body::RigidBody");
    }

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim_name = match animation_name.as_str() {
            "death" => "dead",
            "dead" => "dead",
            _ => "idle",
        };

        play_animation(&self.sprite_name, next_anim_name, 1000, Direction::East, true);
    }

    fn receive_event(&self, _evt: Event) -> () {}

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
