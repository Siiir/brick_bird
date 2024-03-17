//! Module containing plugins related to the in-game simulation.

pub mod state;

pub mod observator;
pub use observator::{bundles::ObservatorBundle, ObservationPlugin};

pub mod hero;
pub use hero::{
    bundles::HeroBundle,
    compos::HeroCore,
    events::{death::HeroDeath, hop::HeroHop},
    res::color::HeroColor,
    HeroPlugin,
};

pub mod obstacles;
pub use obstacles::ObstaclesPlugin;

pub mod plane;
pub use plane::sector::Sector;

pub mod motion;
pub use motion::{
    compos::{acceleration::Acceleration, gravitation::Gravitation, velocity::Velocity},
    res::{gravity::Gravity, scale::MotionScale},
    MotionPlugin,
};

#[deprecated]
#[allow(unused)]
pub mod emotions;
#[deprecated]
pub mod finish;
#[deprecated]
pub mod start;
#[allow(deprecated)]
pub use emotions::EmotionsPlugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{SimulPlanePlugin, SimulStatePlugin};

/// Is meant to contain all the simulation plugins.
///
/// These plugins can be configured, set from these interface,
///  before running the simulation.
#[allow(missing_docs)]
#[derive(Default)]
pub struct SimulPlugins {
    pub observation: ObservationPlugin,
    pub hero: HeroPlugin,
    pub obstacles: ObstaclesPlugin,
    pub simul_plane: SimulPlanePlugin,
    pub motion: MotionPlugin,
    pub simul_state: SimulStatePlugin,
}

impl PluginGroup for SimulPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self {
            observation,
            hero,
            obstacles,
            simul_plane: plane,
            motion,
            simul_state,
        } = self;
        PluginGroupBuilder::start::<Self>()
            .add(observation)
            .add(hero)
            .add(obstacles)
            .add(plane)
            .add(motion)
            .add(simul_state)
    }
}
