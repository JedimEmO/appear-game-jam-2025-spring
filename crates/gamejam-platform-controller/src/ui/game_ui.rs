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

    El::<Node>::new()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align_content(Align::new().bottom())
        .child_signal(health_signal.dedupe().map(move |(hp, max_hp)| {
            let (image_node, _) = sprite_collection
                .create_ui_node_animation_bundle(
                    "ui_healthbar",
                    "idle",
                    Duration::from_millis(5000),
                    true,
                    false,
                    false,
                )
                .expect("failed to open ui_healthbar");

            let children = create_hearts(&sprite_collection, hp, max_hp);

            Stack::<Node>::new()
                .width(Val::Px(256.))
                .height(Val::Px(64.))
                .align_content(Align::new().center_y())
                .layer(El::<ImageNode>::new().image_node(image_node))
                .layer(
                    Row::<ImageNode>::new()
                        .with_node(|mut n| n.padding.left = Val::Px(15.))
                        .items(children),
                )
        }))
        .spawn(world);
}

fn create_hearts(sprite_collection: &SpriteCollection, hp: u32, max_hp: u32) -> Vec<El<ImageNode>> {
    let mut children = vec![];

    let full_hearts = hp / 2;
    let half_hearts = hp % 2;
    let empty_hearts = (max_hp - hp) / 2;

    for _ in 0..full_hearts {
        let (image_node, _anim) = sprite_collection
            .create_ui_node_animation_bundle(
                "item_heart",
                "full",
                Duration::from_millis(5000),
                true,
                false,
                false,
            )
            .expect("failed to open ui_healthbar");

        children.push(El::<ImageNode>::new().image_node(image_node));
    }

    for _ in 0..half_hearts {
        let (image_node, _anim) = sprite_collection
            .create_ui_node_animation_bundle(
                "item_heart",
                "half",
                Duration::from_millis(5000),
                true,
                false,
                false,
            )
            .expect("failed to open ui_healthbar");

        children.push(El::<ImageNode>::new().image_node(image_node));
    }

    for _ in 0..empty_hearts {
        let (image_node, _anim) = sprite_collection
            .create_ui_node_animation_bundle(
                "item_heart",
                "empty",
                Duration::from_millis(5000),
                true,
                false,
                false,
            )
            .expect("failed to open ui_healthbar");

        children.push(El::<ImageNode>::new().image_node(image_node));
    }
    children
}
