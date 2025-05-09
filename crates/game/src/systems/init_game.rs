use avian2d::prelude::*;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gamejam_platform_controller::{GameStates, PlatformerPlugin};

pub struct SimplePlatformGame;

impl Plugin for SimplePlatformGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().with_length_unit(16.),
            PlatformerPlugin,
            LdtkPlugin,
        ))
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(2)
        .add_systems(
            FixedUpdate,
            (wall_spawn_system).run_if(in_state(GameStates::GameLoop)),
        )
        .insert_resource(Gravity(Vec2::new(0., -9.81 * 32.)));

        #[cfg(feature = "inspector")]
        app.add_plugins(WorldInspectorPlugin::new());

        #[cfg(feature = "avian-debug")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

fn wall_spawn_system(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    wall_query: Query<(Entity, &Wall, &GridCoords), Added<Wall>>,
) {
    let Ok(level_id) = level_query.get_single() else {
        return;
    };

    let mut wall_tiles = HashSet::new();
    let mut min_pos = (i32::MAX, i32::MAX);
    let mut max_pos = (i32::MIN, i32::MIN);

    for (_entity, _wall, coords) in wall_query.iter() {
        wall_tiles.insert(coords);

        if min_pos.0 > coords.x {
            min_pos.0 = coords.x;
        }

        if min_pos.1 > coords.y {
            min_pos.1 = coords.y;
        }

        if max_pos.0 < coords.x {
            max_pos.0 = coords.x;
        }

        if max_pos.1 < coords.y {
            max_pos.1 = coords.y;
        }
    }

    let wall_copy = wall_tiles.clone();
    let neighbours =  [
        GridCoords::new(-1, 0),
        GridCoords::new(1, 0),
        GridCoords::new(0, -1),
        GridCoords::new(0, 1),
    ];

    wall_tiles.retain(|tile| {
        for neighbour in neighbours.iter() {
            if !wall_copy.contains(&(*neighbour + **tile)) {
                return true;
            }
        }

        false
    });

    if wall_tiles.is_empty() {
        return;
    }

    for y in min_pos.1..=max_pos.1 {
        let mut strip_start_x = None;

        for x in min_pos.0..=max_pos.0 {
            let pos = GridCoords::new(x, y);
            let is_current_pos_a_tile = wall_tiles.contains(&pos);

            if strip_start_x.is_none() && is_current_pos_a_tile {
                strip_start_x = Some(pos);
            }

            if !is_current_pos_a_tile || x == max_pos.0 {
                if strip_start_x.is_some() {
                    let width = (pos.x - strip_start_x.unwrap().x) as f32;

                    let mut collider = commands.spawn((
                        Transform::from_xyz(
                            16. * (strip_start_x.unwrap().x as f32 + width / 2.),
                            16. * pos.y as f32 + 8.,
                            0.,
                        ),
                        Collider::rectangle(width * 16., 16.),
                        CollisionLayers::new(0b00100, 0b01101),
                        CollidingEntities::default(),
                        RigidBody::Static,
                        Friction::new(0.),
                    ));

                    collider.set_parent(level_id);

                    strip_start_x = None;
                }
            }
        }
    }
}
