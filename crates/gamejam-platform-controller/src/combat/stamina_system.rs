use crate::combat::combat_components::Stamina;
use bevy::prelude::{Query, Res, Time};

pub fn stamina_system(time: Res<Time>, mut stamina_query: Query<&mut Stamina>) {
    for mut stamina in stamina_query.iter_mut() {
        stamina.tick_timer.tick(time.delta());

        stamina.newly_consumed_stamina =
            (stamina.newly_consumed_stamina as f32 * (1. - 0.15* time.delta_secs())) as u32;

        if stamina.tick_timer.just_finished() {
            if stamina.current_stamina < stamina.max_stamina {
                stamina.current_stamina += 1;
            }
        }
    }
}
