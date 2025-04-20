use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use game_entity_component::gamejam::game::game_host::*;
use game_entity_component::{export, exports};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

const WOUND_UP_ATTACK_DELAY_TIMER: u32 = 3000;
const ATTACK_COOLDOWN_TIMER: u32 = 3001;
const TURN_TIMER: u32 = 3002;

export!(EntityWorld);

struct EntityWorld;

impl Guest for EntityWorld {
    type GameEntity = EggProjectileScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct Sounds {
    explode_sound: String,
}

struct EggProjectileScript {
    entity_id: u64,
    animation_info: AnimationInfo,
    state: Cell<EggState>,
    sounds: Sounds,
}

impl EggProjectileScript {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);
        let attack_sound = params.get_parameter::<String>("explode-sound").unwrap();

        let out = Self {
            entity_id: settings.self_entity_id,
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(EggState::Idle),

            sounds: Sounds {
                explode_sound: attack_sound.clone(),
            },
        };
        insert_components(&[InsertableComponents::Attackable, InsertableComponents::Health(1)]);
        out.enter_state(EggState::Idle);

        out
    }
}

pub struct AnimationInfo {
    pub sprite_name: String,
    pub idle_animation: String,
    pub explode_animation: String,
}

impl TryFrom<&ScriptParams> for AnimationInfo {
    type Error = ();

    fn try_from(params: &ScriptParams) -> Result<Self, Self::Error> {
        Ok(Self {
            sprite_name: params.get_parameter("sprite-name").ok_or(())?,
            idle_animation: params.get_parameter("idle-animation").ok_or(())?,
            explode_animation: params.get_parameter("explode-animation").ok_or(())?,
        })
    }
}

#[derive(Copy, Clone)]
enum EggState {
    Idle,
}

impl EggProjectileScript {
    fn enter_state(&self, state: EggState) {
        self.state.set(state);

        match state {
            EggState::Idle => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.idle_animation,
                500,
                Direction::East,
                true,
            ),
        }
    }
}

impl GuestGameEntity for EggProjectileScript {
    fn tick(&self, _delta_t: f32) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {}

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {
        remove_component("avian2d::dynamics::rigid_body::RigidBody");
        play_animation(
            &self.animation_info.sprite_name,
            &self.animation_info.explode_animation,
            300,
            Direction::East,
            true,
        );
        request_timer_callback(0, 300);
    }

    fn timer_callback(&self, timer: u32) -> () {
        despawn_entity(self.entity_id);
    }
}

fn main() {}
