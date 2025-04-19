use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
use crate::ui::stat_bar::stat_bar;
use bevy::color::palettes::tailwind;
use bevy::color::Color;
use bevy::prelude::{ImageNode, Node, PositionType, Val};
use haalka::element::Element;
use haalka::prelude::{El, Sizeable, Stack};
use std::time::Duration;

pub fn stats_widget(
    sprite_collection: &SpriteCollection,
    player_mutables: PlayerStatsMutable,
) -> impl Element {
    let PlayerStatsMutable {
        stamina, health, ..
    } = player_mutables;

    let (health_bar_backdrop, _) = sprite_collection
        .create_ui_node_animation_bundle(
            "ui_healthbar",
            "idle",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_healthbar");

    let stamina_bar = stat_bar(
        stamina,
        160,
        Val::Px(15.),
        Color::Srgba(tailwind::EMERALD_600),
        Color::Srgba(tailwind::AMBER_300),
        |mut n| {
            n.left = Val::Px(14.);
            n.bottom = Val::Px(13.);
            n.position_type = PositionType::Absolute;
        },
    );

    let health_bar = stat_bar(
        health,
        160,
        Val::Px(30.),
        Color::Srgba(tailwind::RED_600),
        Color::Srgba(tailwind::AMBER_300),
        |mut n| {
            n.left = Val::Px(14.);
            n.bottom = Val::Px(48.);
            n.position_type = PositionType::Absolute;
        },
    );

    let health_backdrop = El::<ImageNode>::new()
        .image_node(health_bar_backdrop)
        .width(Val::Percent(100.))
        .height(Val::Percent(100.));

    Stack::<Node>::new()
        .layer(health_backdrop)
        .layer(stamina_bar)
        .layer(health_bar)
        .width(Val::Px(96. * 2.))
        .height(Val::Px(48. * 2.))
}
