use crate::gamejam::game::game_host::play_music;
use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use script_utils::script_parameters::ScriptParams;
use game_entity_component::*;

struct EntityWorld;

use game_entity_component::exports;
use game_entity_component::gamejam::game::game_host::{get_game_data_kv_int, set_game_data_kv_int};

export!(EntityWorld);

impl Guest for EntityWorld {
    type GameEntity = GameLevelScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct GameLevelScript {
    _self_entity_id: u64,
}

impl GameLevelScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let game_state = get_game_data_kv_int("global-game-music-started").unwrap_or(0);
        
        if let Some(music) = params.get_parameter::<String>("music-file") {
            if game_state == 0 || params.get_parameter::<bool>("force-music").unwrap_or(false)  {
                play_music(&music);
                set_game_data_kv_int("global-game-music-started", 1);
            }
        }

        Self {
            _self_entity_id: self_entity_id,
        }
    }
}

impl GuestGameEntity for GameLevelScript {
    fn tick(&self, delta_t: f32) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {}

    fn receive_event(&self, evt: Event) -> () {}

    fn timer_callback(&self, _timer: u32) -> () {}

    fn receive_entity_event(&self, _: EntityEvent) {}
}

fn main() {}
