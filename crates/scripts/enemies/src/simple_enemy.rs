use game_entity_component::exports::gamejam::game::entity_resource::{
    EntityEvent, Event, GameEntity, Guest, GuestGameEntity, StartupSettings,
};
use game_entity_component::gamejam::game::game_host::*;
use game_entity_component::{export, exports};
use script_utils::player_utils::{get_direction_to_player, get_vec_to_player};
use script_utils::script_parameters::ScriptParams;
use std::cell::Cell;

export!(EntityWorld);

struct EntityWorld;

const STAGGERED_TIMER: u32 = 1;
const ATTACK_TIMER: u32 = 2;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum EnemyState {
    Patrolling,
    Aggressive,
}

impl Guest for EntityWorld {
    type GameEntity = SimpleEnemyScript;

    fn get_entity(params: StartupSettings) -> GameEntity {
        set_ticking(true, Some(500.));

        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct SimpleEnemyScript {
    _self_entity_id: u64,
    sprite_name: String,
    timer: Cell<f32>,
    moving: Cell<bool>,
    staggered: Cell<bool>,
    will_attack: Cell<bool>,
    attacking: Cell<bool>,
    dead: Cell<bool>,
    attacking_direction: Cell<Direction>,
    state: Cell<EnemyState>,
    patrol_direction: Cell<Direction>,
    start_uniform: EntityUniform,
    attack_sound: String,
    death_sound: String,
    hit_sound: String,
}

impl SimpleEnemyScript {
    fn new(settings: StartupSettings) -> Self {
        let StartupSettings {
            params,
            self_entity_id,
        } = settings;

        let params = ScriptParams::new(params);

        let sprite_name = params.get_parameter::<String>("sprite-name").unwrap();
        let attack_sound = params.get_parameter::<String>("attack-sound").unwrap();
        let death_sound = params.get_parameter::<String>("death-sound").unwrap();
        let hit_sound = params.get_parameter::<String>("hit-sound").unwrap();

        insert_components(&[
            InsertableComponents::Enemy,
            InsertableComponents::Collider(Collider {
                width: 32.,
                height: 32.,
                physical: false,
            }),
        ]);

        play_animation(&sprite_name, "run", 1000, get_direction_to_player(), true);

        Self {
            _self_entity_id: self_entity_id,
            sprite_name,
            timer: 0.0.into(),
            moving: false.into(),
            attacking: false.into(),
            will_attack: false.into(),
            staggered: false.into(),
            attacking_direction: Cell::new(Direction::West),
            state: Cell::new(EnemyState::Patrolling),
            patrol_direction: Cell::new(Direction::East),
            start_uniform: get_self_uniform(),
            dead: Cell::new(false),
            attack_sound,
            death_sound,
            hit_sound
        }
    }
}

impl SimpleEnemyScript {
    fn do_move(&self, direction_x: f32) {
        if self.staggered.get() || self.will_attack.get() || self.attacking.get() || self.dead.get()  {
            return;
        }

        if !self.moving.get() {
            self.moving.set(true);
            play_animation(
                &self.sprite_name,
                "run",
                1000,
                get_direction_to_player(),
                true,
            );
        }

        send_input(Input::Movement((direction_x.signum(), 0.)));
    }

    fn in_range_of_player(&self) {
        self.moving.set(false);

        if self.will_attack.get() || self.attacking.get() || self.staggered.get() || self.dead.get()  {
            return;
        }

        self.will_attack.set(true);

        request_timer_callback(ATTACK_TIMER, 100);

        self.attacking_direction.set(get_direction_to_player());
        play_animation(
            &self.sprite_name,
            "idle",
            1000,
            get_direction_to_player(),
            true,
        );
    }

    fn was_attacked(&self) {
        if self.attacking.get() || self.dead.get() {
            return;
        }

        play_sound_once(&self.hit_sound);

        self.stagger(1000, get_direction_to_player());
    }

    fn attack(&self) {
        self.will_attack.set(false);

        if self.staggered.get() || self.dead.get()  {
            return;
        }

        self.attacking.set(true);

        play_animation(
            &self.sprite_name,
            "attack",
            400,
            self.attacking_direction.get(),
            false,
        );

        play_sound_once(&self.attack_sound);

        let self_uniform = get_self_uniform();

        let attack_direction = match get_direction_to_player() {
            Direction::West => (-32., 0.),
            _ => (32., 0.)
        };

        schedule_attack(
            200,
            10,
            1.,
            self_uniform.position,
            attack_direction
        );
    }

    fn stagger(&self, duration: u32, direction: Direction) {
        if self.dead.get() {
            return;
        }

        self.staggered.set(true);
        self.will_attack.set(true);
        self.attacking.set(true);
        play_animation(&self.sprite_name, "run", 200, direction, true);
        request_timer_callback(STAGGERED_TIMER, duration);
    }
}

impl GuestGameEntity for SimpleEnemyScript {
    fn tick(&self, delta_t: f32) -> () {
        let dir = get_vec_to_player();
        self.timer.set(self.timer.get() + delta_t);

        if dir.length() > 100. && self.state.get() == EnemyState::Patrolling {
            let distance_to_patrol_x =
                get_self_uniform().position.0 - self.start_uniform.position.0;

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

            return;
        }

        self.state.set(EnemyState::Aggressive);

        if dir.length() > 30. {
            self.do_move(dir.x.signum());
        } else {
            self.in_range_of_player();
        }
    }

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {
        self.was_attacked();
    }

    fn animation_finished(&self, animation_name: String) -> () {
        let next_anim = match animation_name.as_str() {
            "attack" => {
                self.attacking.set(false);
                self.stagger(500, self.attacking_direction.get());

                return;
            }
            "death" => "dead",
            _ => return
        };

        play_animation(&self.sprite_name, next_anim, 1000, Direction::East, true);
    }

    fn receive_event(&self, evt: Event) -> () {}

    fn timer_callback(&self, timer: u32) -> () {
        match timer {
            STAGGERED_TIMER => {
                self.staggered.set(false);
                self.will_attack.set(false);
                self.attacking.set(false);
            }

            ATTACK_TIMER => {
                self.attack();
            }
            _ => {}
        }
    }

    fn receive_entity_event(&self, event: EntityEvent) {
        match event {
            EntityEvent::Killed => {
                self.dead.set(true);
                set_ticking(false, None);
                play_animation(&self.sprite_name, "death", 1000, Direction::East, false);
                play_sound_once(&self.death_sound);
            }
        }
    }
}

fn main() {
}
