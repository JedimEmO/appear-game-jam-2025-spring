use crate::audio::audio_components::{AudioEffect, AudioLevels, AudioMusic};
use crate::graphics::sprite_collection::SpriteCollection;
use crate::main_menu::main_menu_components::{MainMenuComponent, MenuEntry, UiAudioLevels};
use crate::main_menu::menu_input_system::MenuInput;
use crate::GameStates;
use bevy::color::palettes::tailwind;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use haalka::prelude::*;
use std::time::Duration;

pub fn main_menu_system(
    mut commands: Commands,
    mut reader: EventReader<MenuInput>,
    mut audio_levels: ResMut<AudioLevels>,
    ui_audio_levels: ResMut<UiAudioLevels>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<&MainMenuComponent>,
) {
    let menu = query.single();

    let click = asset_server.load("audio/ui_click.wav");
    let click_dark = asset_server.load("audio/ui_click_dark.wav");
    let start = asset_server.load("audio/ui_start.wav");

    for event in reader.read() {
        match event {
            MenuInput::Up => {
                menu.selected_index
                    .set(menu.selected_index.get().wrapping_sub(1));
                menu.selected_index
                    .set(menu.selected_index.get() % menu.entries.lock_ref().len());

                commands.spawn((
                    AudioPlayer::new(click.clone()),
                    PlaybackSettings::ONCE,
                    AudioEffect,
                ));
            }
            MenuInput::Down => {
                menu.selected_index
                    .set(menu.selected_index.get().wrapping_add(1));
                menu.selected_index
                    .set(menu.selected_index.get() % menu.entries.lock_ref().len());

                commands.spawn((
                    AudioPlayer::new(click.clone()),
                    PlaybackSettings::ONCE,
                    AudioEffect,
                ));
            }
            MenuInput::Activate => {
                let Some(entry) = menu
                    .entries
                    .lock_ref()
                    .get(menu.selected_index.get())
                    .cloned()
                else {
                    continue;
                };

                match entry {
                    MenuEntry::StartGame => {
                        commands.spawn((
                            AudioPlayer::new(start.clone()),
                            PlaybackSettings::ONCE,
                            AudioEffect,
                        ));
                        next_state.set(GameStates::SpawnPlayer);
                    }
                    MenuEntry::Attributions => {
                        commands.spawn((
                            AudioPlayer::new(click_dark.clone()),
                            PlaybackSettings::ONCE,
                            AudioEffect,
                        ));

                        menu.entries
                            .lock_mut()
                            .replace_cloned(MenuEntry::attributions_menu());
                    }
                    MenuEntry::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                    MenuEntry::BackToMain => {
                        commands.spawn((
                            AudioPlayer::new(click_dark.clone()),
                            PlaybackSettings::ONCE,
                            AudioEffect,
                        ));
                        menu.entries
                            .lock_mut()
                            .replace_cloned(MenuEntry::default_menu());
                        menu.selected_index.set(0);
                    }
                    MenuEntry::AttributionsList(_) => {}
                    MenuEntry::Settings => {
                        commands.spawn((
                            AudioPlayer::new(click_dark.clone()),
                            PlaybackSettings::ONCE,
                            AudioEffect,
                        ));
                        menu.entries
                            .lock_mut()
                            .replace_cloned(MenuEntry::settings(ui_audio_levels.as_ref()));
                        menu.selected_index.set(0);
                    }
                    _ => {}
                }
            }
            MenuInput::Left => {
                let entries = menu.entries.lock_ref();

                let Some(entry) = entries.get(menu.selected_index.get()) else {
                    continue;
                };

                commands.spawn((
                    AudioPlayer::new(click_dark.clone()),
                    PlaybackSettings::ONCE,
                    AudioEffect,
                ));

                match entry {
                    MenuEntry::LevelControl { tag, .. } => {
                        match tag {
                            0 => audio_levels.decrease_global(),
                            1 => audio_levels.decrease_music(),
                            2 => audio_levels.decrease_effects(),
                            _ => {}
                        };
                    }
                    _ => {}
                }
            }
            MenuInput::Right => {
                let entries = menu.entries.lock_ref();

                let Some(entry) = entries.get(menu.selected_index.get()) else {
                    continue;
                };

                commands.spawn((
                    AudioPlayer::new(click_dark.clone()),
                    PlaybackSettings::ONCE,
                    AudioEffect,
                ));

                match entry {
                    MenuEntry::LevelControl { tag, .. } => {
                        match tag {
                            0 => audio_levels.increase_global(),
                            1 => audio_levels.increase_music(),
                            2 => audio_levels.increase_effects(),
                            _ => {}
                        };
                    }
                    _ => {}
                }
            }
            MenuInput::Back => {
                commands.spawn((
                    AudioPlayer::new(click_dark.clone()),
                    PlaybackSettings::ONCE,
                    AudioEffect,
                ));
                menu.entries
                    .lock_mut()
                    .replace_cloned(MenuEntry::default_menu());
                menu.selected_index.set(0);
            }
        }
    }
}

pub fn leave_main_menu_system(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuComponent>>,
) {
    let cmp = query.single();

    commands.entity(cmp).despawn_recursive();
}

pub fn enter_main_menu_system(
    world: &mut World,
    params: &mut SystemState<(Res<SpriteCollection>, Res<AssetServer>)>,
) {
    let ((sprite_collection, asset_server)) = {
        let (a) = params.get(world);
        a
    };

    let (title_image, _) = sprite_collection
        .create_ui_node_animation_bundle(
            "main_menu_backdrop",
            "idle",
            Duration::from_millis(5000),
            true,
            false,
            false,
        )
        .expect("failed to open main_menu_backdrop");

    let title_image_node =
        El::<ImageNode>::new().image_node(title_image.with_mode(NodeImageMode::Stretch));

    let cmp = MainMenuComponent::default();

    let ui_root = El::<Node>::new()
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .align(Align::center())
        .child(
            Stack::<Node>::new()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .align(Align::center())
                .layer(title_image_node)
                .layer(menu_entries(
                    cmp.entries.signal_vec_cloned(),
                    cmp.selected_index.read_only(),
                    &asset_server,
                )),
        );

    let song = asset_server.load("audio/approaching_the_green_grass.ogg");

    let entity_handle = { ui_root.spawn(world) };
    let mut commands = world.commands();

    let mut audio_entity =
        commands.spawn((AudioPlayer::new(song), PlaybackSettings::LOOP, AudioMusic));

    audio_entity.set_parent(entity_handle);

    commands.entity(entity_handle).insert(cmp);
}

pub fn menu_entries(
    menu_entries: impl SignalVec<Item = MenuEntry> + Send + 'static,
    selected_index: ReadOnlyMutable<usize>,
    asset_server: &Res<AssetServer>,
) -> impl Element {
    let font = TextFont {
        font: asset_server.load("ui/fonts/kongtext.ttf"),
        font_size: 24.,
        ..default()
    };

    let out = Column::<Node>::new()
        .align_content(Align::center())
        .width(Val::Px(400.))
        .items_signal_vec(menu_entries.enumerate().map(move |(idx, entry)| {
            let color_signal = map_ref! {
                let idx = idx.signal(),
                let selected_index = selected_index.signal() => {
                    if *idx == Some(*selected_index) {
                        TextColor(Color::Srgba(tailwind::GRAY_50))
                    } else {
                        TextColor(Color::Srgba(tailwind::GRAY_500))
                    }
                }
            };

            let font_size_enlarge_signal = map_ref! {
                let idx = idx.signal(),
                let selected_index = selected_index.signal() => {
                    if *idx == Some(*selected_index) {
                        1.1
                    } else {
                        1.
                    }
                }
            };

            match entry {
                MenuEntry::StartGame => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(24. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new("Start Game")),
                MenuEntry::Attributions => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(24. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new("Attributions")),
                MenuEntry::Quit => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(24. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new("Quit")),
                MenuEntry::BackToMain => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(15. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new("Back to Main Menu")),
                MenuEntry::AttributionsList(list) => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(8. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new(list.join("\n"))),
                MenuEntry::LevelControl { name, value, .. } => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(14. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text_signal(
                        value
                            .signal()
                            .map(move |value| Text::new(format!("{name} - [{value}]"))),
                    ),
                MenuEntry::Settings => El::<Text>::new()
                    .text_font_signal(font_size_enlarge_signal.map(clone!((font) move |factor| {
                        font.clone().with_font_size(24. * factor)
                    })))
                    .text_color_signal(color_signal)
                    .text(Text::new("Settings")),
            }
        }));

    El::<Node>::new()
        .align(Align::new().bottom().center_x())
        .height(Val::Percent(30.))
        .child(out)
}

pub fn ui_audio_levels_system(levels: ResMut<AudioLevels>, ui_levels: Res<UiAudioLevels>) {
    if !levels.is_changed() {
        return;
    }

    ui_levels.global.set(levels.global);
    ui_levels.music.set(levels.music);
    ui_levels.effects.set(levels.effects);
}
