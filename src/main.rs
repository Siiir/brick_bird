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
    let default_plugins = DefaultPlugins
        .set(LogPlugin {
            filter: "wgpu_hal=error,game=trace".into(),
            level: Level::WARN,
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some("#bullet_bird_canvas".into()),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        });
    App::new()
        .add_plugins((
            default_plugins,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            fly_b::SimulPlugins::default(),
            fly_b::MiscPlugins::default(),
            fly_b::UiPlugins::default(),
        ))
        .run();
}
