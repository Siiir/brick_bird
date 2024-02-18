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
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn(crate::simul::CameraBundle::new(default()));
        })
        .add_systems(Update, sys::follow_hero);
    }
}

/// Systems that control the effect of observation.
pub mod sys;

pub mod bundles {
    /// -
    use bevy::prelude::*;
    use derive_more::Constructor;

    /// All components of the in-game camera entity.
    #[derive(Bundle, Constructor)]
    pub struct CameraBundle {
        base: Camera2dBundle,
    }
}
