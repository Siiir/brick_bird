pub use sect_pass::res::count::PassedSectCount;
pub mod sect_pass;

pub mod keybinds;

pub use music::MusicPlugin;
pub mod music {
    use bevy::prelude::*;

    #[derive(Default)]
    pub struct MusicPlugin {
        _priv_fields_placeholder: (),
    }
    impl Plugin for MusicPlugin {
        fn build(&self, app: &mut App) {
            app.init_asset::<AudioSource>()
                .add_systems(Startup, sys::start_playing);
        }
    }
    pub mod sys {
        use bevy::prelude::*;

        pub fn start_playing(mut cmds: Commands, asset_serv: Res<AssetServer>) {
            cmds.spawn(AudioBundle {
                source: asset_serv.load("sounds/Roa – Color.mp3"),
                settings: PlaybackSettings::LOOP,
            });
        }
    }
}

use self::sect_pass::SectPassPlugin;
use crate::KeybindsPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

/// Is meant to contain all the "miscellaneous" plugins.
///
/// These plugins can be configured, set from these interface,
///  before running the simulation.
#[allow(missing_docs)]
#[derive(Default)]
pub struct MiscPlugins {
    pub sect_pass: SectPassPlugin,
    pub keybinds: KeybindsPlugin,
    pub music: MusicPlugin,
}

impl PluginGroup for MiscPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self {
            sect_pass,
            keybinds,
            music,
        } = self;
        PluginGroupBuilder::start::<Self>()
            .add(sect_pass)
            .add(keybinds)
            .add(music)
    }
}
