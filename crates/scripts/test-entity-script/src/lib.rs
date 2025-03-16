use std::cell::OnceCell;
use std::sync::{Arc, OnceLock};
use game_entity_component::prelude::*;

struct MyCmp;

export!(MyCmp);

use crate::gamejam::game::game_host::*;

use crate::gamejam::game::game_host;

static mut ACTIVATE_COUNT: u32 = 0;
static SETTINGS: OnceLock<StartupSettings> = OnceLock::new();

impl Guest for MyCmp {
    fn startup(params: StartupSettings) -> u64 {
        SETTINGS.set(params).unwrap();

        play_animation("house_1", "idle", 1000, false, true);

        insert_components(&[InsertableComponents::Interactable(Interactable {
            message: "Hello, world".to_string(),
            range: 50.,
        })]);

        set_ticking(true);
        0
    }

    fn tick() {
        despawn_entity(SETTINGS.get().unwrap().self_entity_id);
    }

    fn interacted() {
        unsafe {
            ACTIVATE_COUNT += 1;

            if ACTIVATE_COUNT % 3 == 0 {
                play_animation("lamp_post", "swinging", 1000, false, false);
            }
        }
    }

    fn animation_finished(animation_name: String) {
        let next_anim_name = match animation_name.as_str() {
            "idle" => "glowing",
            _ => "idle",
        };

        play_animation("house_1", next_anim_name, 1000, false, false);
    }
    fn attacked() {
    }

    fn receive_event(_: game_host::Event) { }
}
