use crate::graphics::sprite_collection::SpriteCollection;
use bevy::prelude::*;
use bevy::prelude::{Commands, Res};

pub fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<SpriteCollection>,
) {
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::End,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..default()
                })
                .with_children(sized_image_child_spawn(
                    "ui/action_ui.png",
                    128.,
                    128.,
                    &asset_server,
                ));
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(64.),
                    bottom: Val::Px(0.),
                    ..default()
                })
                .with_children(sized_image_child_spawn(
                    "ui/healthbar.png",
                    128.,
                    32.,
                    &asset_server,
                ));
        });
}

pub fn sized_image_child_spawn<'a>(
    path: &'a str,
    width: f32,
    height: f32,
    asset_server: &'a Res<AssetServer>,
) -> impl FnOnce(&mut ChildBuilder<'_>) + 'a {
    move |parent| {
        parent
            .spawn(Node {
                width: Val::Px(width),
                height: Val::Px(height),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageNode::new(asset_server.load(path)));
            });
    }
}
