pub mod enemy_state_machine;
pub mod attackable;
pub mod enemy;

use bevy::prelude::*;
use crate::enemies::enemy::spawn_enemy_observer;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_enemy_observer);
    }
}