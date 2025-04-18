use crate::gamejam::game::game_host::grant_player_power;
use crate::gamejam::game::game_host::play_music;
use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use game_entity_component::*;
use script_utils::script_parameters::ScriptParams;

struct EntityWorld;

use game_entity_component::exports;
use game_entity_component::gamejam::game::game_host::{
    Direction, EventData, InsertableComponents, Interactable, despawn_entity, get_game_data_kv_int,
    insert_components, play_animation, play_sound_once, publish_event, request_timer_callback,
    set_game_data_kv_int,
};

export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = PowerupScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct PowerupScript {
    self_entity_id: u64,
    power: String,
    state_variable: String,
    trigger_id: Option<u32>,
}

impl PowerupScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);
        let pickup_text = params.get_parameter::<String>("label");
        let sprite_name = params.get_parameter::<String>("sprite-name").unwrap();
        let power = params.get_parameter::<String>("powerup").unwrap();
        let state_variable = params.get_parameter::<String>("state-variable").unwrap();
        let trigger_id = params.get_parameter::<u32>("trigger-id");

        play_animation(&sprite_name, "idle", 1000, Direction::East, true);

        if get_game_data_kv_int(&state_variable).unwrap_or(0) == 1 {
            despawn_entity(self_entity_id);
        } else {
            insert_components(&[InsertableComponents::Interactable(Interactable {
                message: pickup_text.unwrap_or("".to_string()),
                range: 30.,
            })]);
        }

        Self {
            self_entity_id,
            power,
            state_variable,
            trigger_id,
        }
    }
}

impl GuestGameEntity for PowerupScript {
    fn tick(&self, delta_t: f32) -> () {}

    fn interacted(&self) -> () {
        play_sound_once("audio/lvlup.ogg");
        grant_player_power(&self.power);
        set_game_data_kv_int(&self.state_variable, 1);
        request_timer_callback(0, 100);

        if let Some(val) = self.trigger_id {
            publish_event(Event {
                topic: 1,
                data: EventData::Trigger(val),
            });
        }
    }

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {}

    fn receive_event(&self, evt: Event) -> () {}

    fn timer_callback(&self, _timer: u32) -> () {
        despawn_entity(self.self_entity_id);
    }

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
