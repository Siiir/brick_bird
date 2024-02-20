use crate::KeybindsPlugin;

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
    pub keybinds: KeybindsPlugin,
}

impl PluginGroup for MiscPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let Self {
            sect_pass,
            keybinds,
        } = self;
        PluginGroupBuilder::start::<Self>()
            .add(sect_pass)
            .add(keybinds)
    }
}

pub use sect_pass::res::count::PassedSectCount;
pub mod sect_pass;

pub mod keybinds {
    use bevy::prelude::*;

    use crate::SimulState;

    #[derive(Default)]
    pub struct KeybindsPlugin {
        _priv_fields_placeholder: (),
    }
    impl Plugin for KeybindsPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, sys::hero_hop.run_if(in_state(SimulState::Running)));
        }
    }

    pub mod sys {
        use bevy::prelude::*;

        use crate::simul::HeroHop;

        pub fn hero_hop(kbd_input: Res<Input<KeyCode>>, mut hop_announcer: EventWriter<HeroHop>) {
            if kbd_input.just_pressed(KeyCode::Space) {
                hop_announcer.send(HeroHop::new());
            }
        }
    }
}
