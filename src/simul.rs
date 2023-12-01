//! Module containing plugins related to the in-game simulation.

pub mod observator;
pub use observator::{CameraBundle, ObservationPlugin};

pub mod hero;
pub use hero::{bundles::HeroBundle, compos::HeroCore, HeroPlugin};

pub mod obstacles;
pub use obstacles::ObstaclesPlugin;

pub mod plane;
pub use plane::sector::Sector;

pub mod motion;
pub use motion::{compos::Motion, MotionPlugin};

pub mod emotions;
pub use emotions::EmotionsPlugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::SimulPlanePlugin;

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
    pub plane: SimulPlanePlugin,
    pub motion: MotionPlugin,
    pub emotions: EmotionsPlugin,
}

impl PluginGroup for SimulPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self {
            observation,
            hero,
            obstacles,
            plane,
            motion,
            emotions,
        } = self;
        PluginGroupBuilder::start::<Self>()
            .add(observation)
            .add(hero)
            .add(obstacles)
            .add(plane)
            .add(motion)
            .add(emotions)
    }
}
