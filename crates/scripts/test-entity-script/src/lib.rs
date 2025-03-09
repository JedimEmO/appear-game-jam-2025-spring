wit_bindgen::generate!({
    path: "../../gamejam-platform-controller/src/scripting/components",
    world: "game-entity"
});

struct MyCmp;

export!(MyCmp);

use crate::gamejam::game::game_host::{
    InsertableComponents, Interactable, insert_components, play_animation,
};

static mut ACTIVATE_COUNT: u32 = 0;

impl Guest for MyCmp {
    fn startup() -> u64 {
        insert_components(&[InsertableComponents::Interactable(Interactable {
            message: "Hello, world".to_string(),
            range: 50.,
        })]);
        0
    }

    fn tick() {}

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
}
