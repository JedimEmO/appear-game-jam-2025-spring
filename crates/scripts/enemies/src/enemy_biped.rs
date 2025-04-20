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
    type GameEntity = BipedEnemy;

    fn get_entity(params: StartupSettings) -> GameEntity {
        set_ticking(true, Some(500.));

        insert_components(&[
            InsertableComponents::Enemy(game_entity_component::gamejam::game::game_host::Enemy { max_hp: 30 }),
            InsertableComponents::Collider(Collider {
                width: 32.,
                height: 64.,
                physical: false,
            }),
        ]);

        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct EnemyStats {
    aggro_range: f32,
    attack_range: f32,
    windup_attack_delay: u32,
    attack_duration: u32,
    attack_damage: u32,
    attack_force: f32,
}

struct Sounds {
    attack_sound: String,
    death_sound: String,
    hit_sound: String,
}

struct BipedEnemy {
    animation_info: AnimationInfo,
    state: Cell<BipedEnemyStates>,
    patrol_direction: Cell<Direction>,
    on_attack_cooldown: Cell<bool>,
    prewound_charge: Cell<bool>,
    is_dead: Cell<bool>,
    start_uniform: EntityUniform,
    stats: EnemyStats,
    sounds: Sounds,
}

impl BipedEnemy {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);
        let attack_sound = params.get_parameter::<String>("attack-sound").unwrap();
        let death_sound = params.get_parameter::<String>("death-sound").unwrap();
        let hit_sound = params.get_parameter::<String>("hit-sound").unwrap();

        let out = Self {
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(BipedEnemyStates::Patrolling),
            patrol_direction: Cell::new(Direction::West),
            on_attack_cooldown: Cell::new(false),
            prewound_charge: Cell::new(false),
            is_dead: Cell::new(false),
            start_uniform: get_self_uniform(),
            stats: EnemyStats {
                attack_range: params.get_parameter("attack-range").unwrap_or(48.),
                aggro_range: params.get_parameter("aggro-range").unwrap_or(150.),
                windup_attack_delay: params.get_parameter("windup-attack-delay").unwrap_or(500),
                attack_duration: params.get_parameter("attack-duration").unwrap_or(300),
                attack_damage: params.get_parameter("attack-damage").unwrap_or(20),
                attack_force: params.get_parameter("attack-force").unwrap_or(5.),
            },

            sounds: Sounds {
                attack_sound: attack_sound.clone(),
                death_sound: death_sound.clone(),
                hit_sound: hit_sound.clone(),
            },
        };

        out.enter_state(BipedEnemyStates::Patrolling);

        out
    }
}

pub struct AnimationInfo {
    pub sprite_name: String,
    pub idle_animation: String,
    pub walking_animation: String,
    pub windup_animation: String,
    pub wound_animation: String,
    pub attack_animation: String,
    pub staggered_animation: String,
    pub death_animation: String,
    pub dead_animation: String,
}

impl TryFrom<&ScriptParams> for AnimationInfo {
    type Error = ();

    fn try_from(params: &ScriptParams) -> Result<Self, Self::Error> {
        Ok(Self {
            sprite_name: params.get_parameter("sprite-name").ok_or(())?,
            idle_animation: params.get_parameter("idle-animation").ok_or(())?,
            walking_animation: params.get_parameter("walking-animation").ok_or(())?,
            windup_animation: params.get_parameter("windup-animation").ok_or(())?,
            wound_animation: params.get_parameter("wound-animation").ok_or(())?,
            attack_animation: params.get_parameter("attack-animation").ok_or(())?,
            death_animation: params.get_parameter("death-animation").ok_or(())?,
            dead_animation: params.get_parameter("dead-animation").ok_or(())?,
            staggered_animation: params.get_parameter("staggered-animation").ok_or(())?,
        })
    }
}

#[derive(Copy, Clone)]
enum BipedEnemyStates {
    Idle,
    Patrolling,
    Charging,
    WindingUpAttack,
    WoundUp,
    Attacking,
    Dying,
    Dead,
    Staggered,
}

impl BipedEnemy {
    fn enter_state(&self, state: BipedEnemyStates) {
        self.state.set(state);

        match state {
            BipedEnemyStates::Idle => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.idle_animation,
                1000,
                get_self_uniform().facing,
                true,
            ),
            BipedEnemyStates::Patrolling => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.walking_animation,
                600,
                get_self_uniform().facing,
                true,
            ),
            BipedEnemyStates::Charging => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.walking_animation,
                600,
                get_self_uniform().facing,
                true,
            ),
            BipedEnemyStates::WindingUpAttack => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.windup_animation,
                200,
                get_self_uniform().facing,
                false,
            ),
            BipedEnemyStates::Dying => {
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.death_animation,
                    600,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.death_sound);
            }
            BipedEnemyStates::Dead => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.dead_animation,
                10000,
                get_self_uniform().facing,
                true,
            ),
            BipedEnemyStates::WoundUp => {
                request_timer_callback(WOUND_UP_ATTACK_DELAY_TIMER, self.stats.windup_attack_delay);
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.wound_animation,
                    100,
                    get_self_uniform().facing,
                    true,
                )
            }
            BipedEnemyStates::Attacking => {
                let uniform = get_self_uniform();
                let player = get_vec_to_player().normalize() * self.stats.attack_range;

                self.on_attack_cooldown.set(true);

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
            BipedEnemyStates::Staggered => {
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.staggered_animation,
                    600,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.hit_sound);
            }
        }
    }

    fn patrol(&self) {
        let player_vec = get_vec_to_player();
        let self_uniform = get_self_uniform();

        let direction_to_player = if player_vec.x.signum() < 0. {
            Direction::West
        } else {
            Direction::East
        };

        if player_vec.length() < self.stats.aggro_range
            && direction_to_player == self_uniform.facing
        {
            self.enter_state(BipedEnemyStates::Charging);
            return;
        }

        let distance_to_patrol_x = get_self_uniform().position.0 - self.start_uniform.position.0;

        let dir = match self.patrol_direction.get() {
            Direction::East => {
                if distance_to_patrol_x > 50. {
                    self.patrol_direction.set(Direction::West);
                }
                (1., 0.)
            }
            _ => {
                if distance_to_patrol_x < -50. {
                    self.patrol_direction.set(Direction::East);
                }

                (-1., 0.)
            }
        };

        send_input(Input::Movement(dir));
    }

    pub fn charge(&self) {
        let player_vec = get_vec_to_player();

        if player_vec.length() < self.stats.attack_range {
            if self.prewound_charge.get() {
                self.prewound_charge.set(false);
                self.enter_state(BipedEnemyStates::Attacking);
            } else if !self.on_attack_cooldown.get() {
                self.enter_state(BipedEnemyStates::WindingUpAttack);
            }

            return;
        } else {
            send_input(Input::Movement((player_vec.x.signum(), 0.)))
        }
    }

    pub fn face_player(&self) {
        face_direction(get_direction_to_player());
    }
}

impl GuestGameEntity for BipedEnemy {
    fn tick(&self, _delta_t: f32) -> () {
        match self.state.get() {
            BipedEnemyStates::Patrolling => self.patrol(),
            BipedEnemyStates::Charging => self.charge(),
            _ => {
                if get_self_uniform().facing != get_direction_to_player() {
                    request_timer_callback(TURN_TIMER, 500);
                }
            }
        }
    }

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {
        self.enter_state(BipedEnemyStates::Staggered)
    }

    fn animation_finished(&self, animation_name: String) -> () {
        if self.is_dead.get() {
            self.enter_state(BipedEnemyStates::Dead);
        } else if animation_name == self.animation_info.death_animation {
            self.enter_state(BipedEnemyStates::Dead);
        } else if animation_name == self.animation_info.windup_animation {
            self.enter_state(BipedEnemyStates::WoundUp);
        } else if animation_name == self.animation_info.attack_animation {
            request_timer_callback(ATTACK_COOLDOWN_TIMER, 500);
            self.enter_state(BipedEnemyStates::Charging);
        } else if animation_name == self.animation_info.staggered_animation {
            request_timer_callback(ATTACK_COOLDOWN_TIMER, 500);
            self.enter_state(BipedEnemyStates::Charging);
        }
    }

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {
        match evt {
            EntityEvent::Killed => {
                set_ticking(false, None);
                self.is_dead.set(true);
                self.enter_state(BipedEnemyStates::Dying);
            }
        }
    }

    fn timer_callback(&self, timer: u32) -> () {
        match timer {
            // attack is primed, handle various cases wrt range, facing etc. before following through
            WOUND_UP_ATTACK_DELAY_TIMER => {
                if get_vec_to_player().length() > self.stats.attack_range {
                    self.prewound_charge.set(true);
                    self.enter_state(BipedEnemyStates::Charging);
                    return;
                }

                if get_direction_to_player() != get_self_uniform().facing {
                    request_timer_callback(
                        WOUND_UP_ATTACK_DELAY_TIMER,
                        self.stats.windup_attack_delay,
                    );

                    return;
                }

                self.on_attack_cooldown.set(true);
                self.enter_state(BipedEnemyStates::Attacking);
            }
            ATTACK_COOLDOWN_TIMER => {
                self.on_attack_cooldown.set(false);
            }
            TURN_TIMER => {
                self.face_player();
            }
            _ => {}
        }
    }
}

fn main() {}
