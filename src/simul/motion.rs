//! Encapsulates the logic of simple motions of in-simulation objects.

use bevy::prelude::*;

/// Enables simple motion effects.
#[derive(Default)]
pub struct MotionPlugin {
    _future_priv_fields: (),
}
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            .register_type::<compos::Velocity>()
            // Update
            .add_systems(Update, (sys::perform,));
    }
}

/// Components
pub mod compos {
    use bevy::prelude::*;
    use derive_more::{Deref, DerefMut, From, Into};

    /// The entity equiped in this effect will involuntary move forward in uniform motion.
    #[derive(Default, Clone, Copy, From, Component, Reflect, Debug, Deref, DerefMut, Into)]
    #[reflect(Component)]
    pub struct Velocity(Vec2);

    impl From<[f32; 2]> for Velocity {
        fn from([x, y]: [f32; 2]) -> Self {
            Vec2::new(x, y).into()
        }
    }
    impl Into<[f32; 2]> for Velocity {
        fn into(self) -> [f32; 2] {
            Vec2::from(self).into()
        }
    }
}

pub mod sys {
    use bevy::prelude::*;

    pub fn perform(time: Res<Time>, mut bodies: Query<(&mut Transform, &crate::simul::Velocity)>) {
        for (mut transform, motion) in &mut bodies {
            let street = Vec2::from(*motion) * time.delta_seconds();
            transform.translation += street.extend(0.);
        }
    }
}
