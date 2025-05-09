use crate::audio::game_audio_plugin::GameAudioPlugin;
use crate::combat::EnemyPlugin;
use crate::game_entities::file_formats::game_entity_definitions::GameEntityDefinitionFile;
use crate::game_resources::{load_resources, load_scripts_system};
use crate::graphics::animation_system::animated_sprite_system;
use crate::graphics::materials::fog_material::FogMaterial;
use crate::graphics::sprite_collection::{
    spawn_sprite_collection_system, AnimatedSpriteFile, SpriteCollection,
};
use crate::input_systems::gamepad_input::gamepad_input_system;
use crate::input_systems::input_plugin::InputPlugin;
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::ldtk_entities::GameLdtkEntitiesPlugin;
use crate::main_menu::main_menu_plugin::MainMenuPlugin;
use crate::movement_systems::movement_plugin::MovementPlugin;
use crate::player_systems::player_attack_system::{player_attack_start_system, player_pogo_system};
use crate::player_systems::player_control_system::player_control_system;
use crate::player_systems::player_health::player_health_sync_system;
use crate::player_systems::player_spawn_system::{
    spawn_player_system, spawn_player_ui_proxy_system,
};
use crate::scripting::ScriptedGameEntityPlugin;
use crate::timing::timer_system::timer_system;
use crate::ui::game_ui::setup_game_ui;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_rand::plugin::EntropyPlugin;
use bevy_wasmer_scripting::wasm_script_asset::{
    WasmScriptModuleBytes, WasmScriptModuleBytesLoader,
};
use bevy_wasmer_scripting::WasmtimeScriptPlugin;
use gamejam_bevy_components::Interactable;
use haalka::HaalkaPlugin;
use input_systems::PlayerInputAction;
use simple_2d_camera::PixelCameraResolution;
use std::time::Duration;
use crate::levels::levels_plugin::LevelsPlugin;

pub mod audio;
pub mod combat;
pub mod game_entities;
pub mod game_resources;
pub mod graphics;
mod input_systems;
pub mod ldtk_entities;
pub mod main_menu;
pub mod movement_systems;
mod player_const_rules;
pub mod player_systems;
pub mod scripting;
pub mod timing;
pub mod ui;
pub mod levels;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    LoadingSprites,
    #[default]
    LoadingScripts,
    Loading,
    SpawnPlayer,
    MainMenu,
    GameLoop,
    LoadLevel
}

pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app
            //shaders and stuff
            .add_plugins(Material2dPlugin::<FogMaterial>::default())
            // other stuff
            .init_state::<GameStates>()
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_plugins(EntropyPlugin::<bevy_rand::prelude::ChaCha8Rng>::default())
            .add_plugins(GameAudioPlugin)
            .add_plugins(LevelsPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(MainMenuPlugin {})
            .add_plugins(GameLdtkEntitiesPlugin)
            .add_plugins(WasmtimeScriptPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(HaalkaPlugin)
            .add_plugins(ScriptedGameEntityPlugin)
            .add_systems(Startup, (load_resources, spawn_player_ui_proxy_system))
            .init_asset::<WasmScriptModuleBytes>()
            .init_asset_loader::<WasmScriptModuleBytesLoader>()
            .add_plugins((
                TomlAssetPlugin::<AnimatedSpriteFile>::new(&["sprites.toml"]),
                TomlAssetPlugin::<GameEntityDefinitionFile>::new(&["entities.toml"]),
            ))
            .add_systems(
                FixedUpdate,
                load_scripts_system.run_if(in_state(GameStates::LoadingScripts)),
            )
            .add_systems(
                FixedUpdate,
                spawn_sprite_collection_system.run_if(in_state(GameStates::LoadingSprites)),
            )
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::MainMenu)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(OnEnter(GameStates::MainMenu), spawn_fog_system)
            .add_systems(
                OnEnter(GameStates::SpawnPlayer),
                (spawn_player_system, setup_game_ui),
            )
            .add_event::<PlayerInputAction>()
            .add_systems(
                FixedUpdate,
                (
                    animated_sprite_system,
                    player_control_system,
                    player_attack_start_system,
                    player_pogo_system,
                    player_health_sync_system,
                    timer_system,
                )
                    .chain()
                    .run_if(in_state(GameStates::GameLoop)),
            );
    }
}

fn spawn_fog_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_resolution: Res<PixelCameraResolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fog_material_assets: ResMut<Assets<FogMaterial>>,
    camera: Query<Entity, With<Camera2d>>,
) {
    let camera = camera.single();

    let fog_image = asset_server.load("textures/fog.png");

    let fog_material = fog_material_assets.add(FogMaterial {
        color_texture: fog_image,
    });
    let mesh = Mesh::from(Rectangle::new(camera_resolution.0.x, camera_resolution.0.y));
    let mesh = meshes.add(mesh);

    let mut camera_child = commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(fog_material),
        Transform::from_xyz(0., 0., -90.),
    ));

    camera_child.set_parent(camera);
}

#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 4,
        rows = 6,
        padding_x = 2,
        padding_y = 2
    ))]
    player_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "sprites/guy.png")]
    player: Handle<Image>,
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
    terminal: Terminal,
}

fn spawn_thing_system(
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform), Added<Thing>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.z = 2.;

        commands.entity(entity).insert(
            assets
                .create_sprite_animation_bundle(
                    "bounce_fly",
                    "idle",
                    Duration::from_secs(1),
                    true,
                    false,
                    false,
                )
                .expect("missing bounce fly idle animation"),
        );
    }
}

fn spawn_terminal_system(
    mut commands: Commands,
    assets: Res<SpriteCollection>,
    mut query: Query<(Entity, &mut Transform), Added<Terminal>>,
) {
    for (entity, _transform) in query.iter_mut() {
        commands
            .entity(entity)
            .insert(
                assets
                    .create_sprite_animation_bundle(
                        "terminal",
                        "idle",
                        Duration::from_secs(1),
                        true,
                        false,
                        false,
                    )
                    .unwrap(),
            )
            .insert(Interactable {
                action_hint: "press <up> to read terminal".to_string(),
                range: 10.0,
            });
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AttackDirection {
    Down,
    Sideways,
}
