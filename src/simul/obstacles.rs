pub use pole::Pole;
pub mod pole;

use bevy::prelude::*;

#[derive(Default)]
pub struct ObstaclesPlugin {
    _future_priv_fields: (),
}

impl Plugin for ObstaclesPlugin {
    fn build(&self, _app: &mut App) {
        /* This method is here for future use. */
    }
}
