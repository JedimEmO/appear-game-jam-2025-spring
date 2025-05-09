use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_trauma_shake::{Shake, TraumaPlugin};

pub const CAMERA_TRACK_SPEED: f32 = 200.;
pub const CAMERA_TRACK_SPEED_FAST: f32 = 1000.;
pub const SPEED_CAMERA_TRACK_FACTOR: f32 = 0.0;

pub struct SimplePixel2dCameraPlugin {
    pub screen_size: Vec2,
}

impl Default for SimplePixel2dCameraPlugin {
    fn default() -> Self {
        Self {
            screen_size: Vec2::new(480.0, 270.0),
        }
    }
}

#[derive(Resource)]
pub struct PixelCameraResolution(pub Vec2);

#[derive(Component, Default)]
pub struct PixelCameraTracked;

#[derive(Component)]
pub struct CameraShake {
    pub started_at: f64,
    pub duration: f64,
    pub intensity: f32,
    pub timer: Timer,
}

impl CameraShake {
    pub fn soft(now: f64, duration: f64) -> Self {
        Self {
            started_at: now,
            duration,
            intensity: 8.0,
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}

impl Plugin for SimplePixel2dCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelCameraResolution(self.screen_size));
        app.add_plugins(TraumaPlugin);
        app.add_systems(Startup, start_camera_system);
        app.add_systems(FixedUpdate, camera_track_system);
    }
}

fn camera_track_system(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    tracked: Query<(&Transform, &LinearVelocity), (With<PixelCameraTracked>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();

    for (transform, velocity) in tracked.iter() {
        let track_point = calculate_camera_track_point(transform, velocity);
        let dx = track_point.x - camera.translation.x;
        let dy = track_point.y - camera.translation.y;

        let speed_window_x = (dx.abs().clamp(30., 150.) - 30.) / 120.;

        if dx.abs() >= 30. {
            camera.translation.x +=
                dx.signum() * CAMERA_TRACK_SPEED_FAST * speed_window_x * time.delta_secs();
        }

        if velocity.y < -16. * 28. {
            camera.translation.y = transform.translation.y;
        } else if dy.abs() >= 64. {
            camera.translation.y += dy.signum() * CAMERA_TRACK_SPEED * time.delta_secs();
        }
    }
}

fn start_camera_system(
    mut commands: Commands,
    camera_resolution: Res<PixelCameraResolution>,
    asset_server: Res<AssetServer>,
) {
    let sprite = Sprite::from_image(asset_server.load("sprites/scenery/nexus_bg.png"));

    let camera_id = commands
        .spawn((
            Camera2d,
            OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: camera_resolution.0.x,
                    height: camera_resolution.0.y,
                },
                near: -1000.,
                far: 1000.,
                ..OrthographicProjection::default_2d()
            },
            Shake::default(),
            IsDefaultUiCamera,
        ))
        .id();

    let mut camera_child = commands.spawn((sprite, Transform::from_xyz(0., 0., -100.)));
    camera_child.set_parent(camera_id);
}

fn calculate_camera_track_point(transform: &Transform, velocity: &LinearVelocity) -> Vec2 {
    let x_speed = velocity.x;

    let track_x = transform.translation.x + x_speed * SPEED_CAMERA_TRACK_FACTOR;
    let track_y = transform.translation.y;

    Vec2::new(track_x, track_y)
}
