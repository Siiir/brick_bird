use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub use simul::{
    plane::{res::plane::SimulPlane, SimulPlanePlugin},
    SimulPlugins,
};
mod simul;

use bevy::{
    input::common_conditions::input_toggle_active,
    log::{Level, LogPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "wgpu_hal=error,game=debug".into(),
                level: Level::WARN,
            }),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            SimulPlugins::default(),
        ))
        .run();
}
