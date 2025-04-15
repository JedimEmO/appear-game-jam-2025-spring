use bevy::prelude::*;
use haalka::prelude::{Mutable, MutableVec, ReadOnlyMutable};

#[derive(Component)]
pub struct MainMenuComponent {
    pub entries: MutableVec<MenuEntry>,
    pub selected_index: Mutable<usize>,
}

#[derive(Resource, Default)]
pub struct UiAudioLevels {
    pub global: Mutable<f32>,
    pub music: Mutable<f32>,
    pub effects: Mutable<f32>,
}

impl Default for MainMenuComponent {
    fn default() -> Self {
        Self {
            entries: MutableVec::new_with_values(MenuEntry::default_menu()),
            selected_index: Default::default(),
        }
    }
}

impl MenuEntry {
    pub fn attributions_menu() -> Vec<Self> {
        vec![
            MenuEntry::AttributionsList(vec![
                "Additional samples by Ove Melaa (Omsofware@hotmail.com) -2013 Ove Melaa: ".to_string(),
                "\"Approaching the Green Grass\" written and produced by Ove Melaa (Omsofware@hotmail.com) ".to_string(),
                "\"Earth is All we have\" written and produced by Ove Melaa (Omsofware@hotmail.com) ".to_string(),
            ]),
            MenuEntry::BackToMain,
        ]
    }

    pub fn default_menu() -> Vec<Self> {
        vec![
            MenuEntry::StartGame,
            MenuEntry::Settings,
            MenuEntry::Attributions,
            MenuEntry::Quit,
        ]
    }

    pub fn settings(levels: &UiAudioLevels) -> Vec<Self> {
        vec![
            Self::LevelControl {
                value: levels.global.read_only(),
                tag: 0,
                name: "Audio level".to_string(),
            },
            Self::LevelControl {
                value: levels.music.read_only(),
                tag: 1,
                name: "Music level".to_string(),
            },
            Self::LevelControl {
                value: levels.effects.read_only(),
                tag: 2,
                name: "Effects level".to_string(),
            },
            Self::BackToMain
        ]
    }
}

#[derive(Clone)]
pub enum MenuEntry {
    StartGame,
    Attributions,
    Settings,
    Quit,
    BackToMain,
    AttributionsList(Vec<String>),
    LevelControl {
        tag: u32,
        name: String,
        value: ReadOnlyMutable<f32>,
    },
}
