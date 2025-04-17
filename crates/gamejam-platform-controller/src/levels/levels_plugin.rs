use avian2d::prelude::{Physics, PhysicsTime};
use crate::GameStates;
use bevy::prelude::*;
use crate::player_systems::player_components::Player;

const LEVEL_TRANSITION_TIME: f32 = 0.5;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameStates::LoadLevel),
            enter_level_transition_system,
        )
        .add_systems(FixedUpdate, level_change_fade_system);
    }
}

#[derive(Component)]
struct LevelTransitionOverlay {
    entered_at: f32,
}

fn enter_level_transition_system(
    mut commands: Commands,
    time: Res<Time>,
    mut physics_time: ResMut<Time<Physics>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    physics_time.pause();
    let rect = meshes.add(Rectangle::new(500000., 500000.));

    commands.spawn((
        LevelTransitionOverlay {
            entered_at: time.elapsed_secs(),
        },
        Mesh2d(rect),
        MeshMaterial2d(materials.add(Color::Srgba(Srgba::new(0.0, 0.0, 0.0, 1.)))),
        Transform::from_xyz(0., 0., 20.),
    ));
}

fn level_change_fade_system(
    mut commands: Commands,
    time: Res<Time>,
    mut physics_time: ResMut<Time<Physics>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut query: Query<(Entity, &LevelTransitionOverlay, &mut Transform), Without<Camera2d>>,
    camera: Query<&Transform, With<Camera2d>>,
    player: Query<&Player>
) {
    let Ok((entity, transition_overlay, mut transform)) = query.get_single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera.get_single() else {
        return;
    };

    transform.translation.x = camera_transform.translation.x;
    transform.translation.y = camera_transform.translation.y;

    if time.elapsed_secs() - transition_overlay.entered_at >= LEVEL_TRANSITION_TIME {
        physics_time.unpause();
        commands.entity(entity).despawn_recursive();
    }

    if player.is_empty() {
        next_state.set(GameStates::SpawnPlayer);
    } else {
        next_state.set(GameStates::GameLoop);
    }
}
