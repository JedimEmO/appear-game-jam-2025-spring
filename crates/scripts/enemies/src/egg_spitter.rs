use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use game_entity_component::gamejam::game::game_host::*;
use game_entity_component::{export, exports};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

const SPIT_INTERVAL: u32 = 2000;

export!(EntityWorld);

struct EntityWorld;

impl Guest for EntityWorld {
    type GameEntity = EggSpitterEntity;

    fn get_entity(params: StartupSettings) -> GameEntity {
        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct Sounds {
    attack_sound: String,
}

struct EggSpitterEntity {
    drop_offset: u32,
    animation_info: AnimationInfo,
    state: Cell<SpringerEnemyStates>,
    sounds: Sounds,
}

impl EggSpitterEntity {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);
        let attack_sound = params.get_parameter::<String>("attack-sound").unwrap();

        let out = Self {
            drop_offset: params.get_parameter::<u32>("drop-offset").unwrap_or(0),
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(SpringerEnemyStates::Idle),

            sounds: Sounds {
                attack_sound: attack_sound.clone(),
            },
        };

        out.enter_state(SpringerEnemyStates::Idle);

        if out.drop_offset == 0 {
            request_timer_callback(0, SPIT_INTERVAL);
        } else {
            request_timer_callback(1, out.drop_offset);
        }

        out
    }
}

pub struct AnimationInfo {
    pub sprite_name: String,
    pub idle_animation: String,
    pub attack_animation: String,
}

impl TryFrom<&ScriptParams> for AnimationInfo {
    type Error = ();

    fn try_from(params: &ScriptParams) -> Result<Self, Self::Error> {
        Ok(Self {
            sprite_name: params.get_parameter("sprite-name").ok_or(())?,
            idle_animation: params.get_parameter("idle-animation").ok_or(())?,
            attack_animation: params.get_parameter("attack-animation").ok_or(())?,
        })
    }
}

#[derive(Copy, Clone)]
enum SpringerEnemyStates {
    Idle,
    Attacking,
}

impl EggSpitterEntity {
    fn enter_state(&self, state: SpringerEnemyStates) {
        self.state.set(state);

        match state {
            SpringerEnemyStates::Idle => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.idle_animation,
                500,
                get_self_uniform().facing,
                true,
            ),
            SpringerEnemyStates::Attacking => {
                play_sound_once(&self.sounds.attack_sound);
            }
        }
    }
}

impl GuestGameEntity for EggSpitterEntity {
    fn tick(&self, _delta_t: f32) -> () {}

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {}

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {}

    fn timer_callback(&self, timer: u32) -> () {
        if timer == 0 {
            spawn_projectile(Vector { x: 0.0, y: -120. }, Vector { x: 0.0, y: -12. }, "egg_projectile", &[]);
        }

        request_timer_callback(0, SPIT_INTERVAL);
    }
}

fn main() {}
