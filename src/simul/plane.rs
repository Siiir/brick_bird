//! Encapsulates the logic behind the plane that is traversed by the in-game hero.

pub mod res;
pub mod sector;
pub mod sys;
pub mod err {
    pub use super::res::plane::err::{NotSpawned, SectorXMissing};
}

use bevy::prelude::*;

/// Provides the simulation plane with all strictly embeded objects.
#[derive(Default)]
pub struct SimulPlanePlugin {
    _future_priv_fields: (),
}
impl Plugin for SimulPlanePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(crate::SimulPlane::new_open());

        // CRUD-C: Startup
        app.add_systems(
            OnEnter(crate::SimulState::Startup),
            (sys::reset_logical_plane, crate::SimulPlane::spawn).chain(),
        );
        // CRUD-U: Update
        app.add_systems(
            Update,
            (sys::advance,).run_if(crate::SimulState::is_running_cond()),
        );
        // CRUD-D: Cleanup
        app.add_systems(
            OnEnter(crate::SimulState::Cleanup),
            crate::SimulPlane::despawn,
        );
        // CRUD-D: App exit
        app.add_systems(Update, (sys::run_special_drop_of_sects,));
    }
}
