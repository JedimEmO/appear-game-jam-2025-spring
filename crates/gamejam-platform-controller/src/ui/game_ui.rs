use crate::combat::combat_components::BossHealth;
use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
use crate::ui::boss_health::boss_health_bar;
use crate::ui::powerup_ui::powerup_widget;
use crate::ui::stats_ui::stats_widget;
use bevy::ecs::system::SystemState;
use bevy::prelude::Res;
use bevy::prelude::*;
use haalka::prelude::*;

pub fn setup_game_ui(
    world: &mut World,
    params: &mut SystemState<(
        Res<SpriteCollection>,
        Res<BossHealth>,
        Query<&PlayerStatsMutable>,
    )>,
) {
    let (sprite_collection, boss_health, player_mutable_stats) = {
        let (a, b, c) = params.get(world);
        (a.clone(), b, c.single().clone())
    };

    Column::<Node>::new()
        .ui_root()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align_content(Align::new().bottom())

        .items([
            Row::<Node>::new().item(boss_health_bar(boss_health.as_ref())),
            Row::<Node>::new()
                .item(
                    stats_widget(&sprite_collection, player_mutable_stats.clone())
                        .align(Align::new().left()),
                )
                .item(
                    powerup_widget(&sprite_collection, &player_mutable_stats)
                        .align(Align::new().right()),
                ),
        ])
        .spawn(world);
}
