use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_components::PlayerStatsMutable;
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

    let PlayerStatsMutable { hearts, .. } = player_health;
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

    let (empty_heart, _anim) = sprite_collection
        .create_ui_node_animation_bundle(
            "item_heart",
            "empty",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_healthbar");
    let (half_heart, _anim) = sprite_collection
        .create_ui_node_animation_bundle(
            "item_heart",
            "half",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_healthbar");

    let (full_heart, _anim) = sprite_collection
        .create_ui_node_animation_bundle(
            "item_heart",
            "full",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open ui_healthbar");

    let hearts_signal_vec = hearts.signal_vec_cloned().map(move |v| {
        El::<ImageNode>::new().image_node_signal(v.signal().dedupe().map({
            let full_heart = full_heart.clone();
            let half_heart = half_heart.clone();
            let empty_heart = empty_heart.clone();

            move |heart_value| match heart_value {
                2 => full_heart.clone(),
                1 => half_heart.clone(),
                _ => empty_heart.clone(),
            }
        }))
    });

    let health_bar = El::<ImageNode>::new()
        .image_node(health_bar_backdrop)
        .width(Val::Px(256.))
        .height(Val::Px(64.))
        .align_content(Align::new().center_y())
        .child(
            Row::<Node>::new()
                .with_node(|mut n| n.padding.left = Val::Px(15.))
                .items_signal_vec(hearts_signal_vec),
        );

    El::<Node>::new()
        .ui_root()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align_content(Align::new().bottom())
        .child(health_bar)
        .spawn(world);
}
