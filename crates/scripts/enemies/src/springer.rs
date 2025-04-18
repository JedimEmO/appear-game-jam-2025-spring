use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use game_entity_component::gamejam::game::game_host::*;
use game_entity_component::{export, exports};
use script_utils::player_utils::{get_direction_to_player, get_vec_to_player};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

const WOUND_UP_ATTACK_DELAY_TIMER: u32 = 3000;
const ATTACK_COOLDOWN_TIMER: u32 = 3001;
const TURN_TIMER: u32 = 3002;

export!(EntityWorld);

struct EntityWorld;

impl Guest for EntityWorld {
    type GameEntity = SpringerEnemy;

    fn get_entity(params: StartupSettings) -> GameEntity {
        set_ticking(true, Some(64.));

        insert_components(&[
            InsertableComponents::Enemy,
            InsertableComponents::Collider(Collider {
                width: 32.,
                height: 32.,
                physical: false,
            }),
        ]);

        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct EnemyStats {
    attack_range: f32,
    attack_duration: u32,
    attack_damage: u32,
    attack_force: f32,
}

struct Sounds {
    attack_sound: String,
    death_sound: String,
    hit_sound: String,
}

struct SpringerEnemy {
    animation_info: AnimationInfo,
    state: Cell<SpringerEnemyStates>,
    is_dead: Cell<bool>,
    stats: EnemyStats,
    sounds: Sounds,
}

impl SpringerEnemy {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);
        let attack_sound = params.get_parameter::<String>("attack-sound").unwrap();
        let death_sound = params.get_parameter::<String>("death-sound").unwrap();
        let hit_sound = params.get_parameter::<String>("hit-sound").unwrap();

        let out = Self {
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(SpringerEnemyStates::Idle),
            is_dead: Cell::new(false),
            stats: EnemyStats {
                attack_range: params.get_parameter("attack-range").unwrap_or(32.),
                attack_duration: params.get_parameter("attack-duration").unwrap_or(300),
                attack_damage: params.get_parameter("attack-damage").unwrap_or(10),
                attack_force: params.get_parameter("attack-force").unwrap_or(15.),
            },

            sounds: Sounds {
                attack_sound: attack_sound.clone(),
                death_sound: death_sound.clone(),
                hit_sound: hit_sound.clone(),
            },
        };

        out.enter_state(SpringerEnemyStates::Idle);

        out
    }
}

pub struct AnimationInfo {
    pub sprite_name: String,
    pub idle_animation: String,
    pub attack_animation: String,
    pub death_animation: String,
    pub dead_animation: String,
}

impl TryFrom<&ScriptParams> for AnimationInfo {
    type Error = ();

    fn try_from(params: &ScriptParams) -> Result<Self, Self::Error> {
        Ok(Self {
            sprite_name: params.get_parameter("sprite-name").ok_or(())?,
            idle_animation: params.get_parameter("idle-animation").ok_or(())?,
            attack_animation: params.get_parameter("attack-animation").ok_or(())?,
            death_animation: params.get_parameter("death-animation").ok_or(())?,
            dead_animation: params.get_parameter("dead-animation").ok_or(())?,
        })
    }
}

#[derive(Copy, Clone)]
enum SpringerEnemyStates {
    Idle,
    Attacking,
    Dead,
}

impl SpringerEnemy {
    fn enter_state(&self, state: SpringerEnemyStates) {
        self.state.set(state);

        match state {
            SpringerEnemyStates::Idle => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.idle_animation,
                1000,
                get_self_uniform().facing,
                true,
            ),
            SpringerEnemyStates::Dead => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.dead_animation,
                10000,
                get_self_uniform().facing,
                true,
            ),
            SpringerEnemyStates::Attacking => {
                let uniform = get_self_uniform();
                let player = get_vec_to_player().normalize() * self.stats.attack_range;

                schedule_attack(
                    self.stats.attack_duration / 2,
                    self.stats.attack_damage,
                    self.stats.attack_force,
                    uniform.position,
                    (player.x, player.y),
                );

                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.attack_animation,
                    self.stats.attack_duration,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.attack_sound);
            }
        }
    }

    fn patrol(&self) {
        self.face_player();
        let player_vec = get_vec_to_player();

        if player_vec.length() < self.stats.attack_range {
            self.enter_state(SpringerEnemyStates::Attacking);
            return;
        }
    }

    pub fn face_player(&self) {
        face_direction(get_direction_to_player());
    }
}

impl GuestGameEntity for SpringerEnemy {
    fn tick(&self, _delta_t: f32) -> () {
        if matches!(self.state.get(), SpringerEnemyStates::Idle) {
            self.patrol();
        }
    }

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {
        if self.is_dead.get() {
            self.enter_state(SpringerEnemyStates::Dead);
        } else if animation_name == self.animation_info.attack_animation {
            self.enter_state(SpringerEnemyStates::Dead);
        }
    }

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {
        match evt {
            EntityEvent::Killed => {
                set_ticking(false, None);
                self.is_dead.set(true);
                self.enter_state(SpringerEnemyStates::Dead);
            }
        }
    }

    fn timer_callback(&self, timer: u32) -> () {}
}

fn main() {}
