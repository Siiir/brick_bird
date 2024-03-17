//! Encapsulates the logic of simple motions of in-simulation objects.

use bevy::prelude::*;

use crate::simul::{Gravitation, Gravity, MotionScale, Velocity};

/// Enables simple motion effects.
#[derive(Default)]
pub struct MotionPlugin {
    _future_priv_fields: (),
}
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .register_type::<Gravity>()
            .init_resource::<Gravity>()
            .register_type::<MotionScale>()
            .init_resource::<MotionScale>()
            // Components
            .register_type::<Velocity>()
            .register_type::<Gravitation>()
            // Update
            .add_systems(
                Update,
                (
                    sys::accelerate,
                    sys::traverse,
                    sys::get_gravitated,
                    sys::scale_motion,
                ),
            );
    }
}

/// Components
pub mod compos;

/// Resources
pub mod res;

pub mod sys;
