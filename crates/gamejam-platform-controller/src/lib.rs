use std::time::Duration;
use crate::graphics::animation_system::animated_sprite_system;
use crate::graphics::sprite_collection::{setup_sprite_load_system, spawn_sprite_collection_system, AnimatedSpriteFile, SpriteCollection};
use crate::input_systems::gamepad_input::gamepad_input_system;
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::player_systems::grounded_system::grounded_player_system;
use crate::player_systems::movement_dampening_system::movement_dampening_system;
use crate::player_systems::player_attack_system::{player_attack_start_system, player_pogo_system};
use crate::player_systems::player_control_system::player_control_system;
use crate::player_systems::player_spawn_system::{spawn_player_system, spawn_player_ui_proxy_system};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_ecs_ldtk::prelude::*;
use haalka::HaalkaPlugin;
use input_systems::PlayerInputAction;
use player_systems::player_spawn_system;
use crate::enemies::EnemyPlugin;
use crate::ldtk_entities::GameLdtkEntitiesPlugin;
use crate::ldtk_entities::interactable::Interactable;
use crate::player_systems::player_health::player_health_sync_system;
use crate::ui::game_ui::setup_game_ui;

pub mod graphics;
mod input_systems;
pub mod player_components;
mod player_const_rules;
pub mod player_systems;
pub mod ldtk_entities;
pub mod ui;
pub mod enemies;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    LoadingSprites,
    Loading,
    SpawnPlayer,
    GameLoop,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_plugins(GameLdtkEntitiesPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(HaalkaPlugin)
            .add_systems(Startup, (setup_sprite_load_system, spawn_player_ui_proxy_system))
            .add_plugins(TomlAssetPlugin::<AnimatedSpriteFile>::new(&[
                "sprites.toml",
            ]))
            .add_systems(
                Update,
                spawn_sprite_collection_system.run_if(in_state(GameStates::LoadingSprites)),
            )
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::SpawnPlayer)
                    .load_collection::<PlayerAssets>()
                    .load_collection::<ThingAssets>(),
            )
            .add_systems(OnEnter(GameStates::SpawnPlayer), (spawn_player_system, setup_game_ui))
            .add_event::<PlayerInputAction>()
            .add_systems(Update, player_spawn_system::update_player_spawn)
            .add_systems(
                Update,
                (
                    grounded_player_system,
                    player_control_system,
                    movement_dampening_system,
                )
                    .run_if(in_state(GameStates::GameLoop))
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    animated_sprite_system,
                    keyboard_input_system,
                    gamepad_input_system,
                    player_attack_start_system,
                    player_pogo_system,
                    player_health_sync_system
                )
                    .run_if(in_state(GameStates::GameLoop)),
            );
    }
}

#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 6, padding_x = 2, padding_y = 2))]
    player_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "sprites/guy.png")]
    player: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 5, rows = 2))]
    player_attack_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "sprites/attack.png")]
    player_attack: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ThingAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 1))]
    things_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "sprites/thing.png")]
    things: Handle<Image>,
}

#[derive(Default, Component)]
pub struct PlayerSpawnEntity;

#[derive(Bundle, LdtkEntity, Default)]
struct PlayerSpawnEntityBundle {
    player_spawn: PlayerSpawnEntity,
}

#[derive(Default, Component)]
#[require(
    CollisionLayers(|| CollisionLayers::new(0b01000, LayerMask::ALL)),
    Collider(|| Collider::circle(16.)),
)]
struct Thing;

#[derive(Bundle, LdtkEntity, Default)]
struct ThingBundle {
    player_spawn: Thing,
}

#[derive(Default, Component)]
struct Terminal;

#[derive(Bundle, LdtkEntity, Default)]
struct TerminalBundle {
    terminal: Terminal
}

fn spawn_thing_system(
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    query: Query<Entity, Added<Thing>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(assets.create_sprite_animation_bundle(
            "bounce_fly",
            "idle",
            Duration::from_secs(1),
            true,
            false,
            false
        ).expect("missing bounce fly idle animation"));
    }
}

fn spawn_terminal_system(
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform), Added<Terminal>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.z = 0.;

        commands.entity(entity).insert(assets.create_sprite_animation_bundle(
            "terminal",
            "idle",
            Duration::from_secs(1),
            true,
            false,
            false
        ).unwrap()).insert(Interactable {
            action_hint: "press <north> to read terminal".to_string()
        });
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AttackDirection {
    Down,
    Sideways,
}

