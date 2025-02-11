use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_components::PlayerStatsMutable;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res};
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

    let PlayerStatsMutable { hp, max_hp } = player_health;
    let sprite_collection = assets.clone();

    let health_signal = map_ref! {
        let hp = hp.signal(),
        let max_hp = max_hp.signal() => {
            (*hp, *max_hp)
        }
    };

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

    let hearts_signal_vec = health_signal
        .dedupe()
        .map(|(hp, max_hp)| {
            let full_hearts = hp / 2;
            let half_hearts = hp % 2;
            let empty_hearts = (max_hp - hp) / 2;

            let mut hearts = vec![];

            for _ in 0..full_hearts {
                hearts.push(2);
            }

            for _ in 0..half_hearts {
                hearts.push(1);
            }

            for _ in 0..empty_hearts {
                hearts.push(0);
            }

            hearts
        })
        .to_signal_vec()
        .map(move |heart| match heart {
            0 => El::<ImageNode>::new().image_node(empty_heart.clone()),
            1 => El::<ImageNode>::new().image_node(half_heart.clone()),
            _ => El::<ImageNode>::new().image_node(full_heart.clone()),
        });

    let health_bar = El::<ImageNode>::new()
        .image_node(health_bar_backdrop)
        .width(Val::Px(256.))
        .height(Val::Px(64.))
        .align_content(Align::new().center_y())
        .child(
            Row::<ImageNode>::new()
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
