use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
use crate::ui::stat_bar::stat_bar;
use bevy::color::palettes::tailwind;
use bevy::ecs::system::SystemState;
use bevy::prelude::Res;
use bevy::prelude::*;
use haalka::prelude::*;
use std::time::Duration;

pub fn setup_game_ui(
    world: &mut World,
    params: &mut SystemState<(Res<SpriteCollection>, Query<&PlayerStatsMutable>)>,
) {
    let (assets, player_health) = {
        let (a, b) = params.get(world);
        (a.clone(), b.single().clone())
    };

    let PlayerStatsMutable { stamina, health } = player_health;
    let sprite_collection = assets.clone();

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
        180,
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
        180,
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
        .with_node(|mut n| {
            n.left = Val::Px(0.);
            n.bottom = Val::Px(0.);
            n.position_type = PositionType::Absolute;
        })
        .width(Val::Px(256.))
        .height(Val::Px(99.));

    let health_stack = Stack::<Node>::new()
        .layer(health_backdrop)
        .layer(stamina_bar)
        .layer(health_bar)
        .width(Val::Px(256.))
        .height(Val::Px(99.));

    El::<Node>::new()
        .ui_root()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align_content(Align::new().bottom())
        .child(health_stack)
        .spawn(world);
}
