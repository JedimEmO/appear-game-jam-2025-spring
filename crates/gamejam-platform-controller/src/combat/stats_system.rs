use crate::combat::combat_components::{Health, Stamina};
use bevy::prelude::{Query, Res, Time};

pub fn stats_system(time: Res<Time>, mut stamina_query: Query<(&mut Stamina, &mut Health)>) {
    let delta = time.delta();
    for (mut stamina, mut health) in stamina_query.iter_mut() {
        stamina.0.tick(delta);
        health.0.tick(delta);
    }
}
