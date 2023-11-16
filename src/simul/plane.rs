pub mod res;
pub mod sector;
pub mod sys;

use bevy::prelude::*;

#[derive(Default)]
pub struct SimulPlanePlugin {
    _future_priv_fields: (),
}
impl Plugin for SimulPlanePlugin {
    fn build(&self, app: &mut App) {
        // Experiments
        use rand::prelude::*;
        let simul_plane: crate::SimulPlane =
            rand::distributions::Standard.sample(&mut thread_rng());
        app.insert_resource(simul_plane);
        app.add_systems(Startup, (crate::SimulPlane::spawn_sects,));
        app.add_systems(Update, (sys::advance,));
    }
}
