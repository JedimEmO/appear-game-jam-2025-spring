use crate::graphics::sprite_collection::SpriteCollection;
use crate::player_systems::player_components::PlayerStatsMutable;
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

    let PlayerStatsMutable {
        hearts,
        stamina,
        max_stamina,
        newly_consumed_stamina,
        ..
    } = player_health;
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

    let stamina_bar_width_broadcast = map_ref! {
        let current = stamina.signal(),
        let max = max_stamina.signal() => {
            180 * current / max.max(&1)
        }
    }
    .broadcast();

    let newly_used_stamina_bar_broadcast = map_ref! {
        let max = max_stamina.signal(),
        let used = newly_consumed_stamina.signal() => {
            180 * used / max.max(&1)
        }
    }.broadcast();

    let combined_width_signal = map_ref! {
        let a = stamina_bar_width_broadcast.signal(),
        let b = newly_used_stamina_bar_broadcast.signal() => {
            *a + *b
        }
    };

    let newly_used_stamina_bar = El::<Node>::new()
        .width_signal(newly_used_stamina_bar_broadcast.signal().map(|width| Val::Px(width as f32)))
        .height(Val::Px(15.))
        .background_color(BackgroundColor(Color::Srgba(tailwind::AMBER_200)));


    let stamina_bar = El::<Node>::new()
        .width_signal(
            stamina_bar_width_broadcast
                .signal()
                .map(|v| Val::Px(v as f32)),
        )
        .height(Val::Px(15.))
        .background_color(BackgroundColor(Color::Srgba(tailwind::EMERALD_600)));

    let stamina_bar = Row::<Node>::new()
        .with_node(|mut n| {
            n.left = Val::Px(14.);
            n.bottom = Val::Px(13.);
            n.position_type = PositionType::Absolute;
        })
        .width_signal(combined_width_signal.map(|width| Val::Px(width as f32)))
        .item(stamina_bar)
        .item(newly_used_stamina_bar);

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
