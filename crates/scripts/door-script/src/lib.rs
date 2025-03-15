use game_entity_component::prelude::*;
use script_utils::script_parameters::ScriptParams;

struct MyCmp;

export!(MyCmp);

use crate::gamejam::game::game_host;
use crate::gamejam::game::game_host::Collider;
use crate::gamejam::game::game_host::{
    InsertableComponents, Interactable, insert_components, play_animation, remove_component,
};

static mut STATE: u32 = 0;
static mut TRIGGER_VALUE: u32 = 0;

impl Guest for MyCmp {
    fn startup(params: Option<Vec<String>>) -> u64 {
        let params = ScriptParams::new(params);
        unsafe {
            TRIGGER_VALUE = params.get_parameter::<u32>("trigger-id").unwrap();
        }

        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Collider(Collider {
                width: 16.,
                height: 48.,
                physical: true,
            }),
        ]);
        play_animation("door_1", "closed", 1000, false, true);
        0
    }

    fn tick() {}

    fn interacted() {}

    fn animation_finished(animation_name: String) {
        let next_anim_name = match animation_name.as_str() {
            "opening" => "open",
            _ => {
                if unsafe { STATE } == 0 {
                    "closed"
                } else {
                    "open"
                }
            }
        };

        play_animation("door_1", next_anim_name, 1000, false, true);
    }

    fn attacked() {}

    fn receive_event(evt: game_host::Event) {
        match evt.data {
            game_host::EventData::Trigger(id) => unsafe {
                if id == unsafe { TRIGGER_VALUE } && STATE == 0 {
                    STATE = 1;
                    remove_component("avian2d::dynamics::rigid_body::RigidBody");
                    play_animation("door_1", "opening", 1000, false, false);
                }
            },
        }
    }
}
