use crate::enemies::EnemyPlugin;
use crate::game_entities::file_formats::game_entity_definitions::GameEntityDefinitionFile;
use crate::game_resources::{load_resources, load_scripts_system};
use crate::graphics::animation_system::animated_sprite_system;
use crate::graphics::materials::fog_material::FogMaterial;
use crate::graphics::sprite_collection::{
    spawn_sprite_collection_system, AnimatedSpriteFile, SpriteCollection,
};
use crate::input_systems::gamepad_input::gamepad_input_system;
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::ldtk_entities::GameLdtkEntitiesPlugin;
use crate::player_systems::grounded_system::grounded_player_system;
use crate::player_systems::movement_dampening_system::movement_dampening_system;
use crate::player_systems::player_attack_system::{player_attack_start_system, player_pogo_system};
use crate::player_systems::player_control_system::player_control_system;
use crate::player_systems::player_health::player_health_sync_system;
use crate::player_systems::player_spawn_system::{
    spawn_player_system, spawn_player_ui_proxy_system,
};
use crate::scripting::scripted_game_entity::wasmwat_system;
use crate::ui::game_ui::setup_game_ui;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_wasmer_scripting::wasm_script_asset::{
    WasmScriptModuleBytes, WasmScriptModuleBytesLoader,
};
use bevy_wasmer_scripting::WasmtimeScriptPlugin;
use gamejam_bevy_components::Interactable;
use haalka::HaalkaPlugin;
use input_systems::PlayerInputAction;
use simple_2d_camera::PixelCameraResolution;
use std::time::Duration;
use bevy::sprite::Material2dPlugin;
use crate::scripting::ScriptedGameEntityPlugin;

pub mod enemies;
pub mod game_entities;
pub mod game_resources;
pub mod graphics;
mod input_systems;
pub mod ldtk_entities;
pub mod player_components;
mod player_const_rules;
pub mod player_systems;
pub mod scripting;
pub mod ui;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    LoadingSprites,
    #[default]
    LoadingScripts,
    Loading,
    SpawnPlayer,
    GameLoop,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //shaders and stuff
            .add_plugins(Material2dPlugin::<FogMaterial>::default())
            // other stuff
            .insert_resource(
                FramepaceSettings::default().with_limiter(Limiter::from_framerate(60.)),
            )
            .add_plugins(bevy_framepace::FramepacePlugin)
            .init_state::<GameStates>()
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_plugins(GameLdtkEntitiesPlugin)
            .add_plugins(WasmtimeScriptPlugin)
            .add_plugins(EnemyPlugin)
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
                Update,
                load_scripts_system.run_if(in_state(GameStates::LoadingScripts)),
            )
            .add_systems(
                Update,
                spawn_sprite_collection_system.run_if(in_state(GameStates::LoadingSprites)),
            )
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::SpawnPlayer)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(
                OnEnter(GameStates::SpawnPlayer),
                (spawn_player_system, setup_game_ui, spawn_fog_system),
            )
            .add_event::<PlayerInputAction>()
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
                    player_health_sync_system,
                    wasmwat_system,
                )
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
        color_texture: fog_image
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
    query: Query<Entity, Added<Thing>>,
) {
    for entity in query.iter() {
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
