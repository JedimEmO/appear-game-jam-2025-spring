use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    Collider, Direction, EventData, InsertableComponents, get_game_data_kv_int, insert_components,
    play_animation, remove_component, set_game_data_kv_int,
};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = PlatformScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct PlatformScript {
    _self_entity_id: u64,
    trigger_value: u32,
    state_variable: String,
    sprite: String,
    width: f32,
    invert: bool,
}

impl PlatformScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let trigger_value = params.get_parameter::<u32>("trigger-id").unwrap();
        let sprite = params.get_parameter::<String>("sprite-name").unwrap();
        let state_variable = params.get_parameter::<String>("state-variable").unwrap();
        let width = params.get_parameter::<f32>("platform-width").unwrap_or(64.);
        let invert = params.get_parameter::<bool>("invert").unwrap_or(false);

        let out = Self {
            _self_entity_id: self_entity_id,
            trigger_value,
            state_variable,
            sprite,
            width,
            invert,
        };

        if out.is_solid() {
            insert_components(&[InsertableComponents::Collider(Collider {
                width,
                height: 16.,
                physical: true,
            })]);
            play_animation(&out.sprite, "solid", 1000, Direction::East, false);
        } else {
            play_animation(&out.sprite, "ghost", 1000, Direction::East, true);
        }

        out
    }

    fn is_solid(&self) -> bool {
        let game_state = get_game_data_kv_int(&self.state_variable).unwrap_or(0);

        if self.invert {
            game_state == 0
        } else {
            game_state == 1
        }
    }
}

impl GuestGameEntity for PlatformScript {
    fn tick(&self, delta_t: f32) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim_name = match animation_name.as_str() {
            "materializing" => "solid",
            _ => {
                if !self.is_solid() {
                    "ghost"
                } else {
                    "solid"
                }
            }
        };

        play_animation(&self.sprite, next_anim_name, 1000, Direction::East, true);
    }

    fn receive_event(&self, evt: Event) -> () {
        match evt.data {
            EventData::Trigger(id) => {
                if id == self.trigger_value {
                    if self.is_solid() {
                        insert_components(&[InsertableComponents::Collider(Collider {
                            width: self.width,
                            height: 16.,
                            physical: true,
                        })]);

                        play_animation(&self.sprite, "materializing", 1000, Direction::East, false);
                    } else {
                        remove_component("avian2d::dynamics::rigid_body::RigidBody");
                        play_animation(&self.sprite, "ghost", 1000, Direction::East, false);
                    }
                }
            }
        }
    }

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
