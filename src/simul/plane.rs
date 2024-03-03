//! Encapsulates the logic behind the plane that is traversed by the in-game hero.

pub mod res;
pub mod sector;
pub mod sys;

use bevy::prelude::*;

/// Provides the simulation plane with all strictly embeded objects.
#[derive(Default)]
pub struct SimulPlanePlugin {
    _future_priv_fields: (),
}
impl Plugin for SimulPlanePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(crate::SimulPlane::new_open());

        // Startup
        app.add_systems(
            OnEnter(crate::SimulState::Startup),
            (sys::reset_logical_plane, crate::SimulPlane::spawn_sects).chain(),
        );
        // Cleanup
        app.add_systems(
            OnEnter(crate::SimulState::Cleanup),
            crate::SimulPlane::despawn_sects,
        );
        // Update
        app.add_systems(
            Update,
            (sys::advance,).run_if(in_state(crate::SimulState::Running)),
        );
    }
}
