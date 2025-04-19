use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
use avian2d::parry::utils::center;
use bevy::prelude::*;
use haalka::prelude::*;
use std::time::Duration;

pub fn powerup_widget(
    sprite_collection: &SpriteCollection,
    player_mutables: &PlayerStatsMutable,
) -> impl Element {
    let (powerup_bg, _) = sprite_collection
        .create_ui_node_animation_bundle(
            "ui_powerup_bg",
            "idle",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_powerup_bg");

    let (pogo_icon, _) = sprite_collection
        .create_ui_node_animation_bundle(
            "ui_pogo_icon",
            "idle",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_pogo_icon");

    let (roll_icon, _) = sprite_collection
        .create_ui_node_animation_bundle(
            "ui_roll_icon",
            "idle",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_roll_icon");

    Stack::<Node>::new()
        .layer(
            El::<ImageNode>::new()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .image_node(powerup_bg),
        )
        .layer(
            El::<Node>::new()
                .width(Val::Px(96. * 2. - 40.))
                .align(Align::center())
                .child(Row::<Node>::new().items([
                    icon_container(roll_icon, player_mutables.has_rolling.signal()),
                    icon_container(pogo_icon, player_mutables.has_pogo.signal()),
                ])),
        )
        .width(Val::Px(96. * 2.))
        .height(Val::Px(48. * 2.))
}
fn icon_container(
    image: ImageNode,
    is_visible_signal: impl Signal<Item = bool> + Send + 'static,
) -> impl Element {
    El::<Node>::new()
        .align_content(Align::center())
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .child(icon(image, is_visible_signal))
}

fn icon(
    image: ImageNode,
    is_visible_signal: impl Signal<Item = bool> + Send + 'static,
) -> impl Element {
    El::<ImageNode>::new()
        .image_node(image)
        .width(Val::Px(64.))
        .height(Val::Px(64.))
        .visibility_signal(is_visible_signal.map(|v| {
            if v {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }))
}
