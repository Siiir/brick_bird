pub use observator::{CameraBundle, ObservationPlugin};
pub mod observator;

pub use hero::{bundles::HeroBundle, compos::HeroCore, HeroPlugin};
pub mod hero;

pub use obstacles::ObstaclesPlugin;
pub mod obstacles;

pub use plane::{res::hero_sect::HeroSect, sector::Sector};
pub mod plane;

pub use motion::{compos::Motion, MotionPlugin};
pub mod motion;

pub use emotions::EmotionsPlugin;
pub mod emotions;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::SimulPlanePlugin;

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
