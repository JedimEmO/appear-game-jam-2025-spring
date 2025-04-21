use bevy::prelude::*;

#[derive(Component)]
pub struct Bonfire {
    pub level_index: u32,
    pub spawn_name: String
}

impl Default for Bonfire {
    fn default() -> Self {
        Self {
            level_index: 0,
            spawn_name: "bonfire".to_string()
        }
    }
}
