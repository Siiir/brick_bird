use bevy::{app::PluginGroupBuilder, prelude::*};

/// Is meant to contain all the UI plugins.
///
/// These plugins can be configured, set from these interface,
///  before running the simulation.
#[allow(missing_docs)]
#[derive(Default)]
pub struct UiPlugins {
    pub stats_show: SimulOverlayPlugin,
}

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self { stats_show } = self;
        PluginGroupBuilder::start::<Self>().add(stats_show)
    }
}

pub use simul_overlay::{
    bundles::{
        motion_scale_disp::MotionScaleDispBundle, node::SimulOverlayBundle,
        passed_sect_count_disp::PassedSectCountDispBundle,
    },
    compos::{motion_scale::MotionScaleDisp, passed_sect_count::PassedSectCountDisp},
    SimulOverlayPlugin,
};
pub mod simul_overlay;
