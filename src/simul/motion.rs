//! Encapsulates the logic of simple motions of in-simulation objects.

use bevy::prelude::*;

/// Enables simple motion effects.
#[derive(Default)]
pub struct MotionPlugin {
    _future_priv_fields: (),
}
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<compos::Motion>();
    }
}

/// Components
pub mod compos {
    use bevy::prelude::*;
    use derive_more::Constructor;
    /// The entity equiped in this effect will involuntary move forward in uniform motion.
    #[derive(Component, Debug, Reflect, Constructor)]
    #[reflect(Component)]
    pub struct Motion {
        pub velocity: f32, // u/s
    }
    /// The initial motion of a hero on the game's start.
    impl Default for Motion {
        fn default() -> Self {
            Self::new(crate::simul::hero::INIT_VELOCITY)
        }
    }
}
