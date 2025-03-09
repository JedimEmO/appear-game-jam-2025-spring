wit_bindgen::generate!({
    path: "../../gamejam-platform-controller/src/scripting/components",
    world: "game-entity"
});

struct MyCmp;

export!(MyCmp);

use crate::gamejam::game::game_host::{insert_components, play_animation, remove_component, InsertableComponents, Interactable};
use crate::gamejam::game::game_host::Collider;

static mut STATE: u32 = 0;

impl Guest for MyCmp {
    fn startup() -> u64 {
        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Interactable(Interactable { message: "Flip for fun".to_string(), range: 16.0 }),
            InsertableComponents::Collider(Collider { width: 24., height: 24.}),
        ]);
        play_animation("lever", "open", 1000, false, true);
        0
    }

    fn tick() {}

    fn interacted() {
        remove_component("gamejam_bevy_components::Interactable");
        remove_component("gamejam_platform_controller::ui::interactable_hint::InteractableHintComponent");

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
            },
        };

        play_animation("lever", next_anim_name, 1000, false, true);
    }

    fn attacked() {
        MyCmp::interacted()
    }
}
