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
    sprite: String,
    state: Cell<i32>,
    width: f32
}

impl GuestGameEntity for DoorScript {
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

        let game_state = get_game_data_kv_int(&state_variable).unwrap_or(0);

        if game_state == 1 {
            insert_components(&[
                InsertableComponents::Collider(Collider {
                    width,
                    height: 16.,
                    physical: true,
                }),
            ]);
            play_animation(&sprite, "solid", 1000, false, true);
        } else {
            play_animation(&sprite, "ghost", 1000, false, true);
        }
        Self {
            _self_entity_id: self_entity_id,
            trigger_value,
            state: Cell::new(game_state),
            state_variable,
            sprite,
            width
        }
    }

    fn tick(&self) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim_name = match animation_name.as_str() {
            "materializing" => "solid",
            _ => {
                if self.state.get() == 0 {
                    "ghost"
                } else {
                    "solid"
                }
            }
        };

        play_animation(&self.sprite, next_anim_name, 1000, false, true);
    }

    fn receive_event(&self, evt: Event) -> () {
        match evt.data {
            EventData::Trigger(id) => {
                if id == self.trigger_value && self.state.get() == 0 {
                    self.state.set(1);

                    insert_components(&[
                        InsertableComponents::Collider(Collider {
                            width: self.width,
                            height: 16.,
                            physical: true,
                        }),
                    ]);

                    play_animation(&self.sprite, "materializing", 1000, false, false);
                }
            }
        }
    }

    fn timer_callback(&self, _timer: u32) -> () {
    }
}
