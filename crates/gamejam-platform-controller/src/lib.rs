use crate::graphics::animation_system::animated_sprite_system;
use crate::input_systems::gamepad_input::gamepad_input_system;
use crate::input_systems::keyboard_input_system::keyboard_input_system;
use crate::player_systems::grounded_system::grounded_system;
use crate::player_systems::movement_dampening_system::movement_dampening_system;
use crate::player_systems::player_attack_system::player_attack_start_system;
use crate::player_systems::player_control_system::player_control_system;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use player_systems::player_spawn_system;
use crate::player_systems::player_spawn_system::spawn_player_system;

pub mod graphics;
mod input_systems;
pub mod player_components;
mod player_const_rules;
pub mod player_systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    Loading,
    SpawnPlayer,
    GameLoop,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::SpawnPlayer)
                    .load_collection::<PlayerAssets>()
                    .load_collection::<ThingAssets>(),
            )
            .add_systems(
                OnEnter(GameStates::SpawnPlayer),
                spawn_player_system,
            )
            .add_event::<PlayerInputAction>()
            .add_systems(Update, player_spawn_system::update_player_spawn)
            .add_systems(
                Update,
                (
                    grounded_system,
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
                    spawn_thing_system,
                    keyboard_input_system,
                    gamepad_input_system,
                    player_attack_start_system,
                ).run_if(in_state(GameStates::GameLoop)),
            );

        setup_ldtk_entities(app);
    }
}

fn setup_ldtk_entities(app: &mut App) {
    app.register_ldtk_entity::<PlayerSpawnEntityBundle>("PlayerSpawn");
    app.register_ldtk_entity::<ThingBundle>("Branch");
}

#[derive(Resource, Default)]
pub struct PlayerSpawnSettings {
    pub position: Vec2,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 6))]
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

fn spawn_thing_system(
    mut commands: Commands,
    assets: Res<ThingAssets>,
    query: Query<Entity, Added<Thing>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Sprite::from_atlas_image(
            assets.things.clone(),
            TextureAtlas::from(assets.things_layout.clone()),
        ));
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AttackDirection {
    Down,
    Sideways,
}

#[derive(Event)]
pub enum PlayerInputAction {
    Horizontal(Vec2),
    Jump,
    JumpAbort,
    Attack(AttackDirection),
}
