//! Here are all the modules that are used by the game.

pub mod color;

#[allow(deprecated)]
pub use simul::{
    finish::{events::SimulFinish, SimulFinishPlugin},
    plane::{res::plane::SimulPlane, SimulPlanePlugin},
    start::{events::SimulStart, SimulStartPlugin},
    state::{states::SimulState, SimulStatePlugin},
    SimulPlugins,
};
pub mod simul;

pub use ui::UiPlugins;
pub mod ui;

pub use misc::{keybinds::KeybindsPlugin, MiscPlugins};
pub mod misc;
