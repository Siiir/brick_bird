//! The fly-bevy game's binary.

#![warn(missing_docs)]

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{
    input::common_conditions::input_toggle_active,
    log::{Level, LogPlugin},
    prelude::*,
};

/// Launches the app logic with simple in-place config.
fn main() {
    // Just add + configure plugins and then run.
    // Plugins should encapsulate all the games logic.
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "wgpu_hal=error,game=debug".into(),
                level: Level::WARN,
            }),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            fly_b::SimulPlugins::default(),
            fly_b::MiscPlugins::default(),
            fly_b::UiPlugins::default(),
        ))
        .run();
}
