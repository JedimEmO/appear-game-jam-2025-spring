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
const EGG_THROW_TIMER: u32 = 3003;
const EGG_RECHARGE_TIMER: u32 = 3004;

export!(EntityWorld);

struct EntityWorld;

impl Guest for EntityWorld {
    type GameEntity = EasterBunnyBossEntity;

    fn get_entity(params: StartupSettings) -> GameEntity {
        set_ticking(true, Some(1000.));

        insert_components(&[
            InsertableComponents::Enemy(game_entity_component::gamejam::game::game_host::Enemy {
                max_hp: 200,
            }),
            InsertableComponents::Boss,
            InsertableComponents::Collider(Collider {
                width: 128.,
                height: 128.,
                physical: false,
            }),
        ]);

        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct EnemyStats {
    aggro_range: f32,
    throw_range: f32,
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

struct EasterBunnyBossEntity {
    animation_info: AnimationInfo,
    state: Cell<BunnyStates>,
    patrol_direction: Cell<Direction>,
    ammunition: Cell<u32>,
    has_reloaded: Cell<bool>,
    on_attack_cooldown: Cell<bool>,
    prewound_charge: Cell<bool>,
    is_dead: Cell<bool>,
    start_uniform: EntityUniform,
    stats: EnemyStats,
    sounds: Sounds,
}

impl EasterBunnyBossEntity {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);

        let out = Self {
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(BunnyStates::Patrolling),
            patrol_direction: Cell::new(Direction::West),
            ammunition: Cell::new(4),
            has_reloaded: Cell::new(true),
            on_attack_cooldown: Cell::new(false),
            prewound_charge: Cell::new(false),
            is_dead: Cell::new(false),
            start_uniform: get_self_uniform(),
            stats: EnemyStats {
                attack_range: params.get_parameter("attack-range").unwrap_or(96.),
                aggro_range: params.get_parameter("aggro-range").unwrap_or(150.),
                windup_attack_delay: params.get_parameter("windup-attack-delay").unwrap_or(500),
                attack_duration: params.get_parameter("attack-duration").unwrap_or(400),
                attack_damage: params.get_parameter("attack-damage").unwrap_or(20),
                attack_force: params.get_parameter("attack-force").unwrap_or(10.),
                throw_range: 300.,
            },

            sounds: Sounds {
                attack_sound: "audio/monsters/bunny_umpf.wav".to_string(),
                death_sound: "audio/monsters/bunny_umpf.wav".to_string(),
                hit_sound: "audio/monsters/bunny_umpf.wav".to_string(),
            },
        };

        out.enter_state(BunnyStates::Patrolling);

        request_timer_callback(EGG_RECHARGE_TIMER, 2000);

        out
    }
}

pub struct AnimationInfo {
    pub sprite_name: String,
    pub idle_animation: String,
    pub walking_animation: String,
    pub windup_animation: String,
    pub wound_animation: String,
    pub throw_animation: String,
    pub attack_animation: String,
    pub staggered_animation: String,
    pub death_animation: String,
    pub dead_animation: String,
}

impl TryFrom<&ScriptParams> for AnimationInfo {
    type Error = ();

    fn try_from(params: &ScriptParams) -> Result<Self, Self::Error> {
        Ok(Self {
            sprite_name: "easter_bunny".to_string(),
            idle_animation: "idle".to_string(),
            walking_animation: "walk".to_string(),
            windup_animation: "windup".to_string(),
            wound_animation: "woundup".to_string(),
            throw_animation: "throw".to_string(),
            attack_animation: "attack".to_string(),
            death_animation: "dying".to_string(),
            dead_animation: "dead".to_string(),
            staggered_animation: "airborne".to_string(),
        })
    }
}

#[derive(Copy, Clone)]
enum BunnyStates {
    Idle,
    Patrolling,
    Charging,
    WindingUpAttack,
    WoundUp,
    Attacking,
    Dying,
    Dead,
    Staggered,
    Throwing,
    Turret,
}

impl EasterBunnyBossEntity {
    fn enter_state(&self, state: BunnyStates) {
        self.state.set(state);

        match state {
            BunnyStates::Idle => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.idle_animation,
                700,
                get_self_uniform().facing,
                true,
            ),
            BunnyStates::Patrolling => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.walking_animation,
                600,
                get_self_uniform().facing,
                true,
            ),
            BunnyStates::Charging => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.walking_animation,
                600,
                get_self_uniform().facing,
                true,
            ),
            BunnyStates::WindingUpAttack => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.windup_animation,
                200,
                get_self_uniform().facing,
                false,
            ),
            BunnyStates::Dying => {
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.death_animation,
                    600,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.death_sound);
            }
            BunnyStates::Dead => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.dead_animation,
                10000,
                get_self_uniform().facing,
                true,
            ),
            BunnyStates::WoundUp => {
                request_timer_callback(WOUND_UP_ATTACK_DELAY_TIMER, self.stats.windup_attack_delay);
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.wound_animation,
                    100,
                    get_self_uniform().facing,
                    true,
                )
            }
            BunnyStates::Attacking => {
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
            BunnyStates::Staggered => {
                self.on_attack_cooldown.set(true);
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.staggered_animation,
                    200,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.hit_sound);
            }
            BunnyStates::Throwing => {
                self.on_attack_cooldown.set(true);
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.throw_animation,
                    300,
                    get_self_uniform().facing,
                    false,
                );
                play_sound_once(&self.sounds.attack_sound);
                request_timer_callback(EGG_THROW_TIMER, 200);
            }
            BunnyStates::Turret => {
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.idle_animation,
                    700,
                    get_self_uniform().facing,
                    true,
                );
            }
        }
    }

    fn idle(&self) {
        let player_vec = get_vec_to_player();

        if player_vec.y.abs() > 128. {
            return;
        }

        let self_uniform = get_self_uniform();

        if self_uniform.facing != get_direction_to_player() {
            self.face_player();
        }

        if player_vec.length() < self.stats.throw_range && self.has_reloaded.get() {
            self.enter_state(BunnyStates::Turret);
            return;
        }

        if player_vec.length() > self.stats.attack_range {
            self.enter_state(BunnyStates::Charging);
            return;
        }
    }

    pub fn turret(&self) {
        if self.ammunition.get() == 0 {
            self.enter_state(BunnyStates::Idle);
        }

        let player_vec = get_vec_to_player();

        if self.has_reloaded.get() {
            if player_vec.length() < self.stats.throw_range {
                if !self.on_attack_cooldown.get() {
                    if self.ammunition.get() > 0 {
                        self.ammunition.set(self.ammunition.get() - 1);
                        self.enter_state(BunnyStates::Throwing);
                    } else {
                        self.has_reloaded.set(false);
                    }
                }
            }

            return;
        }

        self.enter_state(BunnyStates::Idle);
    }

    fn charge(&self) {
        let player_vec = get_vec_to_player();

        if player_vec.length() < self.stats.attack_range {
            self.enter_state(BunnyStates::WindingUpAttack);
            return;
        }

        send_input(Input::Movement((player_vec.x.signum(), 0.)))
    }

    pub fn face_player(&self) {
        face_direction(get_direction_to_player());
    }
}

impl GuestGameEntity for EasterBunnyBossEntity {
    fn tick(&self, _delta_t: f32) -> () {
        match self.state.get() {
            BunnyStates::Idle => self.idle(),
            BunnyStates::Turret => self.turret(),
            BunnyStates::Charging => self.charge(),
            _ => {}
        }
    }

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {
        self.enter_state(BunnyStates::Staggered)
    }

    fn animation_finished(&self, animation_name: String) -> () {
        if self.is_dead.get() {
            self.enter_state(BunnyStates::Dead);
        } else if animation_name == self.animation_info.death_animation {
            self.enter_state(BunnyStates::Idle);
        } else if animation_name == self.animation_info.windup_animation {
            self.enter_state(BunnyStates::Attacking);
        } else if animation_name == self.animation_info.attack_animation {
            request_timer_callback(ATTACK_COOLDOWN_TIMER, 1000);
            self.enter_state(BunnyStates::Idle);
        } else if animation_name == self.animation_info.staggered_animation {
            request_timer_callback(ATTACK_COOLDOWN_TIMER, 500);
            self.enter_state(BunnyStates::Idle);
        } else if animation_name == self.animation_info.throw_animation {
            request_timer_callback(ATTACK_COOLDOWN_TIMER, 1000);
            self.enter_state(BunnyStates::Turret);
        }
    }

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {
        match evt {
            EntityEvent::Killed => {
                set_ticking(false, None);
                self.is_dead.set(true);
                self.enter_state(BunnyStates::Dying);
            }
        }
    }

    fn timer_callback(&self, timer: u32) -> () {
        match timer {
            EGG_RECHARGE_TIMER => {
                request_timer_callback(EGG_RECHARGE_TIMER, 2000);

                if self.has_reloaded.get() {
                    return;
                }

                let was_not_full = self.ammunition.get() < 4;

                if !was_not_full {
                    return;
                }

                self.ammunition.set(self.ammunition.get() + 1);
                let is_full = self.ammunition.get() == 4;

                if was_not_full && is_full {
                    self.has_reloaded.set(true);
                }
            }
            EGG_THROW_TIMER => {
                let player_uni = get_player_uniform();
                let self_uni = get_self_uniform();

                let dx = player_uni.position.0 - self_uni.position.0;
                let dy = player_uni.position.1 - self_uni.position.1;

                let dir = glam::Vec2::new(dx, dy).normalize();

                let x_offset = dir.x * 30.;
                let dir = dir * 220.;

                spawn_projectile(
                    Vector { x: dir.x, y: dir.y },
                    Vector { x: x_offset, y: 0. },
                    "egg_projectile",
                    &[],
                );
            }
            // attack is primed, handle various cases wrt range, facing etc. before following through
            WOUND_UP_ATTACK_DELAY_TIMER => {
                if get_vec_to_player().length() > self.stats.attack_range {
                    self.prewound_charge.set(true);
                    self.enter_state(BunnyStates::Charging);
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
                self.enter_state(BunnyStates::Attacking);
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
