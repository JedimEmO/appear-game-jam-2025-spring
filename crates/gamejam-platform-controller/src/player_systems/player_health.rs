use crate::combat::HitPoints;
use crate::player_systems::player_components::{Player, PlayerStats, PlayerStatsMutable};
use bevy::prelude::{Query, With, Without};
use haalka::prelude::Mutable;
use crate::combat::combat_components::Stamina;

pub fn player_health_sync_system(
    player_stats: Query<&PlayerStatsMutable, Without<Player>>,
    player_hp: Query<(&HitPoints, &PlayerStats, &Stamina), With<Player>>,
) {
    let Ok(hp) = player_hp.get_single() else {
        return;
    };

    let Ok(stats) = player_stats.get_single() else {
        return;
    };

    if hp.0.hp == 0 {
        panic!("Game over man. Game over!");
    }

    stats.hp.set(hp.0.hp);
    stats.max_hp.set(hp.1.max_health);
    stats.stamina.set(hp.2.current_stamina);
    stats.max_stamina.set(hp.2.max_stamina);
    stats.newly_consumed_stamina.set(hp.2.newly_consumed_stamina);

    let mut s = stats.hearts.lock_mut();

    let hearts = stats.max_hp.get() as usize / 2;

    if s.len() < hearts {
        for _i in 0..(hearts - s.len()) {
            s.push_cloned(Mutable::new(0));
        }
    }

    if hearts < s.len() {
        for _i in 0..(s.len() - hearts) {
            s.pop();
        }
    }

    let hp = stats.hp.get();

    let full_hearts = hp as usize / 2;
    let half_hearts = hp as usize % 2;

    for i in 0..hearts {
        if i < full_hearts {
            s[i].set(2);
        } else if i < full_hearts + half_hearts {
            s[i].set(1);
        } else {
            s[i].set(0);
        }
    }
}
