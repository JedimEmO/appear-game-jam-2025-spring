use crate::player_systems::player_components::StatBarMutables;
use bevy::color::Color;
use bevy::prelude::{BackgroundColor, Mut, Node, Val};
use haalka::prelude::{map_ref, El, Row, SignalExt, Sizeable};

pub fn stat_bar(
    stat: StatBarMutables,
    bar_width: u32,
    bar_height: Val,
    main_color: Color,
    used_color: Color,
    f: impl FnOnce(Mut<Node>) + Send + 'static,
) -> Row<Node> {
    let stamina_bar_width_broadcast = map_ref! {
        let current = stat.current.signal(),
        let max = stat.max.signal() => {
            bar_width * current / max.max(&1)
        }
    }
    .broadcast();

    let newly_used_stamina_bar_broadcast = map_ref! {
        let max = stat.max.signal(),
        let used = stat.newly_consumed.signal() => {
            bar_width * used / max.max(&1)
        }
    }
    .broadcast();

    let combined_width_signal = map_ref! {
        let a = stamina_bar_width_broadcast.signal(),
        let b = newly_used_stamina_bar_broadcast.signal() => {
            *a + *b
        }
    };

    let newly_used_bar = El::<Node>::new()
        .width_signal(
            newly_used_stamina_bar_broadcast
                .signal()
                .map(|width| Val::Px(width as f32)),
        )
        .height(bar_height)
        .background_color(BackgroundColor(used_color));

    let bar = El::<Node>::new()
        .width_signal(
            stamina_bar_width_broadcast
                .signal()
                .map(|v| Val::Px(v as f32)),
        )
        .height(bar_height)
        .background_color(BackgroundColor(main_color));

    Row::<Node>::new()
        .with_node(f)
        .width_signal(combined_width_signal.map(|width| Val::Px(width as f32)))
        .item(bar)
        .item(newly_used_bar)
}
