wit_bindgen::generate!({
    path: "../../gamejam-platform-controller/src/scripting/components",
    world: "game-entity"
});

struct MyCmp;


export!(MyCmp);

use crate::gamejam::game::game_host::{InsertableComponents, Interactable, insert_components};

impl Guest for MyCmp {
    fn startup() -> u64 {
        insert_components(&[
            InsertableComponents::Interactable(Interactable {
                message: "Hello, world".to_string(),
                range: 50.
            })
        ]);
        0
    }

    fn tick() {}

    fn interacted() {}
    
    fn animation_finished(animation_name: String) {}
}
