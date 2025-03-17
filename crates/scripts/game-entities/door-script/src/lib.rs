use game_entity_component::exports::gamejam::game::entity_resource::{
    Event, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    get_game_data_kv_int, insert_components, play_animation, remove_component, set_game_data_kv_int,
    Collider, EventData, InsertableComponents,
};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = DoorScript;
}

struct DoorScript {
    _self_entity_id: u64,
    trigger_value: u32,
    state_variable: String,
    state: Cell<i32>,
}

impl GuestGameEntity for DoorScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let trigger_value = params.get_parameter::<u32>("trigger-id").unwrap();
        let state_variable = params.get_parameter::<String>("state-variable").unwrap();

        let game_state = get_game_data_kv_int(&state_variable).unwrap_or(0);

        if game_state == 0 {
            insert_components(&[
                InsertableComponents::Attackable,
                InsertableComponents::Collider(Collider {
                    width: 16.,
                    height: 48.,
                    physical: true,
                }),
            ]);
            play_animation("door_1", "closed", 1000, false, true);
        } else {
            insert_components(&[InsertableComponents::Attackable]);
            play_animation("door_1", "open", 1000, false, true);
        }
        Self {
            _self_entity_id: self_entity_id,
            trigger_value,
            state: Cell::new(game_state),
            state_variable,
        }
    }

    fn tick(&self) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim_name = match animation_name.as_str() {
            "opening" => "open",
            _ => {
                if self.state.get() == 0 {
                    "closed"
                } else {
                    "open"
                }
            }
        };

        play_animation("door_1", next_anim_name, 1000, false, true);
    }

    fn receive_event(&self, evt: Event) -> () {
        match evt.data {
            EventData::Trigger(id) => {
                if id == self.trigger_value && self.state.get() == 0 {
                    self.state.set(1);
                    remove_component("avian2d::dynamics::rigid_body::RigidBody");
                    play_animation("door_1", "opening", 1000, false, false);
                }
            }
        }
    }
}
