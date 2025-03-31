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

impl Guest for EntityWorld {
    type GameEntity = BipedEnemy;

    fn get_entity(params: StartupSettings) -> GameEntity {
        set_ticking(true, Some(500.));

        insert_components(&[
            InsertableComponents::Enemy,
            InsertableComponents::Collider(Collider {
                width: 32.,
                height: 64.,
                physical: false,
            }),
        ]);

        GameEntity::new(Self::GameEntity::new(params))
    }
}

struct BipedEnemy {
    animation_info: AnimationInfo,
    state: Cell<BipedEnemyStates>,
    patrol_direction: Cell<Direction>,
    start_uniform: EntityUniform,
}

impl BipedEnemy {
    pub fn new(settings: StartupSettings) -> Self {
        let params = ScriptParams::new(settings.params);

        let out = Self {
            animation_info: AnimationInfo::try_from(&params).unwrap(),
            state: Cell::new(BipedEnemyStates::Patrolling),
            patrol_direction: Cell::new(Direction::West),
            start_uniform: get_self_uniform(),
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
        })
    }
}

#[derive(Copy, Clone)]
enum BipedEnemyStates {
    Idle,
    Patrolling,
    Charging,
    Attacking,
    Dying,
    Dead,
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
            BipedEnemyStates::Attacking => {}
            BipedEnemyStates::Dying => play_animation(
                &self.animation_info.sprite_name,
                &self.animation_info.death_animation,
                600,
                get_self_uniform().facing,
                false,
            ),
            BipedEnemyStates::Dead => {
                play_animation(
                    &self.animation_info.sprite_name,
                    &self.animation_info.dead_animation,
                    10000,
                    get_self_uniform().facing,
                    true,
                )
            }
        }
    }

    fn patrol(&self) {
        let player_vec = get_vec_to_player();

        // if player_vec.length() < 150. {
        //     self.enter_state(BipedEnemyStates::Charging);
        //     return;
        // }

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
}

impl GuestGameEntity for BipedEnemy {
    fn tick(&self, _delta_t: f32) -> () {
        match self.state.get() {
            BipedEnemyStates::Patrolling => self.patrol(),
            BipedEnemyStates::Charging => self.patrol(),
            _ => {}
        }
    }

    fn interacted(&self) -> () {}

    fn attacked(&self) -> () {}

    fn animation_finished(&self, animation_name: String) -> () {
        if animation_name == self.animation_info.death_animation {
            self.enter_state(BipedEnemyStates::Dead);
        }
    }

    fn receive_event(&self, evt: Event) -> () {}

    fn receive_entity_event(&self, evt: EntityEvent) -> () {
        match evt {
           EntityEvent::Killed => {
               set_ticking(false, None);
               self.enter_state(BipedEnemyStates::Dying);
           }
        }
    }

    fn timer_callback(&self, timer: u32) -> () {}
}

fn main() {}
