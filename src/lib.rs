//! Here are all the modules that are used by the game.

pub mod color;

pub use simul::{
    plane::{res::plane::SimulPlane, SimulPlanePlugin},
    SimulPlugins,
};
mod simul;
