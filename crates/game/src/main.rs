mod systems;

use crate::systems::init_game::SimplePlatformGame;
use bevy::prelude::*;
use simple_2d_camera::SimplePixel2dCameraPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.7)))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SimplePixel2dCameraPlugin::default(),
        ))
        .add_plugins(SimplePlatformGame);

    #[cfg(feature = "fps")]
    app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default());

    app.run();
}
