use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct InteractableHintComponent;

pub fn make_interactable_hint(asset_server: &Res<AssetServer>, hint: String) -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            top: Val::Px(100.),
            ..default()
        },
        InteractableHintComponent,
        Text(hint),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        TextFont {
            font: asset_server.load("ui/fonts/kongtext.ttf"),
            font_size: 15.,
            ..default()
        }
    )
}