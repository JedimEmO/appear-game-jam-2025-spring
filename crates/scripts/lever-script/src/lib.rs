use game_entity_component::prelude::*;
use script_utils::script_parameters::ScriptParams;

struct MyCmp;

export!(MyCmp);

use crate::game_host::publish_event;
use crate::gamejam::game::game_host;
use crate::gamejam::game::game_host::Collider;
use crate::gamejam::game::game_host::{
    InsertableComponents, Interactable, insert_components, play_animation, remove_component,
};

static mut STATE: u32 = 0;
static mut TRIGGER_TARGET: Vec<u32> = vec![];

impl Guest for MyCmp {
    fn startup(params: StartupSettings) -> u64 {
        let StartupSettings {
            params,
            self_entity_id
        } = params;

        let params = ScriptParams::new(params);

        unsafe {
            TRIGGER_TARGET = params.get_list_parameter::<u32>("trigger-targets").unwrap();
        }

        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Collider(Collider {
                width: 24.,
                height: 24.,
                physical: false,
            }),
        ]);
        play_animation("lever", "open", 1000, false, true);
        0
    }

    fn tick() {}

    fn interacted() {
        remove_component("gamejam_bevy_components::Interactable");
        remove_component(
            "gamejam_platform_controller::ui::interactable_hint::InteractableHintComponent",
        );

        unsafe {
            if STATE == 0 {
                STATE = 1;

                play_animation("lever", "closing", 1000, false, false);
            } else if STATE == 1 {
                play_animation("lever", "closed", 1000, false, true);
            }
        }
    }

    fn animation_finished(animation_name: String) {
        let next_anim_name = match animation_name.as_str() {
            "closing" => "closed",
            _ => {
                if unsafe { STATE } == 0 {
                    "open"
                } else {
                    "closed"
                }
            }
        };

        play_animation("lever", next_anim_name, 1000, false, true);
    }

    fn attacked() {
        MyCmp::interacted();
        #[allow(static_mut_refs)]
        unsafe {
            for val in &TRIGGER_TARGET {
                publish_event(game_host::Event {
                    topic: 1,
                    data: game_host::EventData::Trigger(*val),
                });
            }
        }
    }

    fn receive_event(evt: game_host::Event) {}
}
