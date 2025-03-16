use game_entity_component::exports::gamejam::game::entity_resource::{
    Event, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    Collider, EventData, InsertableComponents, insert_components, play_animation, publish_event,
    remove_component,
};
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
    state: Cell<u32>,
}

impl GuestGameEntity for LeverScript {
    fn new(params: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = params;

        let params = ScriptParams::new(params);

        let trigger_targets = params.get_list_parameter::<u32>("trigger-targets").unwrap();

        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Collider(Collider {
                width: 24.,
                height: 24.,
                physical: false,
            }),
        ]);
        play_animation("lever", "open", 1000, false, true);

        Self {
            _self_entity_id: self_entity_id,
            trigger_targets,
            state: Cell::new(0),
        }
    }

    fn tick(&self) {}

    fn interacted(&self) {
        remove_component("gamejam_bevy_components::Interactable");
        remove_component(
            "gamejam_platform_controller::ui::interactable_hint::InteractableHintComponent",
        );

        if self.state.get() == 0 {
            self.state.set(1);

            play_animation("lever", "closing", 1000, false, false);
        } else if self.state.get() == 1 {
            play_animation("lever", "closed", 1000, false, true);
        }
    }

    fn attacked(&self) {
        self.interacted();

        for val in &self.trigger_targets {
            publish_event(Event {
                topic: 1,
                data: EventData::Trigger(*val),
            });
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
}
