use self::sect_pass::SectPassPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

/// Is meant to contain all the "miscellaneous" plugins.
///
/// These plugins can be configured, set from these interface,
///  before running the simulation.
#[allow(missing_docs)]
#[derive(Default)]
pub struct MiscPlugins {
    pub sect_pass: SectPassPlugin,
}

impl PluginGroup for MiscPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self { sect_pass } = self;
        PluginGroupBuilder::start::<Self>().add(sect_pass)
    }
}

pub use sect_pass::res::count::PassedSectCount;
pub mod sect_pass;
pub mod events {}
