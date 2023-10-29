mod simul;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "wgpu_hal=error,game=debug".into(),
            level: Level::WARN,
        }))
        .add_plugins(simul::CameraPlugin::default())
        .add_plugins(simul::HeroPlugin::default())
        .run();
}
