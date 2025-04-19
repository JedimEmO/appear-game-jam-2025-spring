use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
use crate::ui::powerup_ui::powerup_widget;
use crate::ui::stats_ui::stats_widget;
use bevy::ecs::system::SystemState;
use bevy::prelude::Res;
use bevy::prelude::*;
use haalka::prelude::*;

pub fn setup_game_ui(
    world: &mut World,
    params: &mut SystemState<(Res<SpriteCollection>, Query<&PlayerStatsMutable>)>,
) {
    let (sprite_collection, player_mutable_stats) = {
        let (a, b) = params.get(world);
        (a.clone(), b.single().clone())
    };

    Row::<Node>::new()
        .ui_root()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align_content(Align::new().bottom())
        .item(
            stats_widget(&sprite_collection, player_mutable_stats.clone())
                .align(Align::new().left()),
        )
        .item(powerup_widget(&sprite_collection, &player_mutable_stats).align(Align::new().right()))
        .spawn(world);
}
