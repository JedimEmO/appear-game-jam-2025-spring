use game_entity_component::exports::gamejam::game::entity_resource::{
    Event, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

use game_entity_component::gamejam::game::game_host::{
    get_game_data_kv_int, insert_components, play_animation, remove_component,
    set_game_data_kv_int, Collider, EventData, InsertableComponents,
};
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = RubbleScript;
}

struct RubbleScript {
    _self_entity_id: u64,
    sprite_name: String,
    death_animation_duration: u32
}

impl GuestGameEntity for RubbleScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let sprite_name = params.get_parameter::<String>("sprite-name").unwrap();
        let death_animation_duration = params.get_parameter::<u32>("death-duration").unwrap_or(400);

        insert_components(&[
            InsertableComponents::Attackable,
            InsertableComponents::Collider(Collider {
                width: 32.,
                height: 32.,
                physical: false,
            }),
        ]);

        play_animation(&sprite_name, "idle", 1000, false, true);

        Self {
            _self_entity_id: self_entity_id,
            sprite_name,
            death_animation_duration
        }
    }

    fn tick(&self) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {
        play_animation(&self.sprite_name, "death", self.death_animation_duration, false, false);
        remove_component("avian2d::dynamics::rigid_body::RigidBody");
    }

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim_name = match animation_name.as_str() {
            "death" => "dead",
            "dead" => "dead",
            _ => "idle",
        };

        play_animation(&self.sprite_name, next_anim_name, 1000, false, true);
    }

    fn receive_event(&self, _evt: Event) -> () {}
}
