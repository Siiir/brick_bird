//! Module encapsulating the logic behind observation of the in-game simulation through "camera".

use bevy::prelude::*;

/// Plugin that gives a player ability to observe the in-game simulation. Mainly through "camera".
#[derive(Default)]
pub struct ObservationPlugin {
    _future_priv_fields: (),
}

// Impl.
impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (sys::maximize_win, sys::spawn))
            .add_systems(Update, (sys::react_to_window_resize, sys::follow_hero));
    }
}

/// Systems that control the effect of observation.
pub mod sys;

pub mod bundles;
