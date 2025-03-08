use bevy_ecs_macros::Component;
use bevy_reflect::Reflect;

#[derive(Component, Reflect)]
pub struct Interactable {
    pub action_hint: String,
    pub range: f32
}