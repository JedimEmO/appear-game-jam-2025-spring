use bevy::prelude::{Query, With, Without};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use haalka::prelude::Mutable;
use crate::enemies::HitPoints;
use crate::player_components::{Player, PlayerStats, PlayerStatsMutable};

pub fn player_health_sync_system(
    player_stats: Query<(&PlayerStatsMutable), Without<Player>>,
    player_hp: Query<(&HitPoints, &PlayerStats), With<Player>>,
) {
    let Ok(hp) = player_hp.get_single() else {
        return;
    };

    let Ok(stats) = player_stats.get_single() else {
        return;
    };

    stats.hp.set(hp.0.hp);
    stats.max_hp.set(hp.1.max_health);

    let mut s = stats.hearts.lock_mut();

    let hearts = stats.max_hp.get() as usize/ 2;

    if s.len() < hearts {
        for _i in 0 ..(hearts - s.len()) {
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