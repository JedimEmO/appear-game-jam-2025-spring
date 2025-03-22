use game_entity_component::exports::gamejam::game::entity_resource::{
    Event, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;
use std::time::Duration;
use game_entity_component::gamejam::game::game_host::{Collider, EventData, InsertableComponents, insert_components, play_animation, publish_event, set_game_data_kv_int, get_game_data_kv_int, remove_component, request_timer_callback};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = LeverScript;
}

struct LeverScript {
    _self_entity_id: u64,
    trigger_targets: Vec<u32>,
    state_variable: String,
    state: Cell<i32>,
    delay: Option<Duration>,
}

impl GuestGameEntity for LeverScript {
    fn new(params: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = params;

        let params = ScriptParams::new(params);

        let trigger_targets = params.get_list_parameter::<u32>("trigger-targets").unwrap();
        let state_variable = params.get_parameter::<String>("state-variable").unwrap();
        let delay_millis = params.get_parameter::<u32>("delay-millis");
        let game_state = get_game_data_kv_int(&state_variable).unwrap_or(0);


        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Collider(Collider {
                width: 24.,
                height: 24.,
                physical: false,
            }),
        ]);

        let animation = if game_state == 0 {
            "open"
        } else {
            "closed"
        };

        play_animation("lever", animation, 1000, false, true);

        Self {
            _self_entity_id: self_entity_id,
            trigger_targets,
            state_variable,
            state: Cell::new(game_state),
            delay: delay_millis.map(|v| Duration::from_millis(v as u64))
        }
    }

    fn tick(&self) {}

    fn interacted(&self) {
    }

    fn attacked(&self) {
        if let Some(delay) = self.delay.map(|d| d.as_millis()) {
            request_timer_callback(0, delay as u32);
        } else {
            self.activate();
        }
    }

    fn animation_finished(&self, animation_name: String) {
        let next_anim_name = match animation_name.as_str() {
            "closing" => "closed",
            _ => {
                if self.state.get() == 0 {
                    "open"
                } else {
                    "closed"
                }
            }
        };

        play_animation("lever", next_anim_name, 1000, false, true);
    }

    fn receive_event(&self, evt: Event) {}

    fn timer_callback(&self, _timer: u32) -> () {
        self.activate();
    }
}

impl LeverScript {
    fn activate(&self) {
        remove_component("gamejam_bevy_components::Interactable");
        remove_component(
            "gamejam_platform_controller::ui::interactable_hint::InteractableHintComponent",
        );

        if self.state.get() == 0 {
            self.state.set(1);
            set_game_data_kv_int(&self.state_variable, 1);
            play_animation("lever", "closing", 1000, false, false);
        } else if self.state.get() == 1 {
            play_animation("lever", "closed", 1000, false, true);
        }

        for val in &self.trigger_targets {
            publish_event(Event {
                topic: 1,
                data: EventData::Trigger(*val),
            });
        }
    }
}