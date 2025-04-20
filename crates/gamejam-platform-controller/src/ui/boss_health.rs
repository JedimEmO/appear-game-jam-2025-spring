use bevy::color::Color;
use bevy::color::palettes::tailwind;
use bevy::prelude::Node;
use haalka::prelude::SignalExt;
use crate::combat::combat_components::BossHealth;
use bevy::ui::Val;
use haalka::align::{Align, Alignable};
use haalka::element::Element;
use haalka::prelude::{Row, Sizeable};
use crate::ui::stat_bar::stat_bar;

pub fn boss_health_bar(health: &BossHealth) -> impl Element {
    let health_stat = health.0.clone();
    Row::<Node>::new().width(Val::Percent(100.))
        .align_content(Align::center())
        .item_signal(health.1.signal().dedupe().map(move |v| {
            if v {
                Some(stat_bar(
                    health_stat.clone(),
                    200,
                    Val::Px(32.),
                    Color::Srgba(tailwind::RED_600),
                    Color::Srgba(tailwind::AMBER_300),
                    |_| {}
                ))
            } else {
                None
            }
        }))
}
