//! Encapsulates the logic of simple motions of in-simulation objects.

use bevy::prelude::*;

use crate::simul::Velocity;

/// Enables simple motion effects.
#[derive(Default)]
pub struct MotionPlugin {
    _future_priv_fields: (),
}
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            .register_type::<Velocity>()
            // Update
            .add_systems(Update, (sys::accelerate, sys::traverse));
    }
}

/// Components
pub mod compos {
    pub mod velocity {
        use bevy::prelude::*;
        use derive_more::{Deref, DerefMut, From, Into};

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
    pub mod acceleration {
        use bevy::prelude::*;
        use derive_more::{Deref, DerefMut, From, Into};

        #[derive(Default, Clone, Copy, From, Component, Reflect, Debug, Deref, DerefMut, Into)]
        #[reflect(Component)]
        pub struct Acceleration(Vec2);

        impl From<[f32; 2]> for Acceleration {
            fn from([x, y]: [f32; 2]) -> Self {
                Vec2::new(x, y).into()
            }
        }

        impl Into<[f32; 2]> for Acceleration {
            fn into(self) -> [f32; 2] {
                Vec2::from(self).into()
            }
        }
    }
}

pub mod sys {
    use bevy::prelude::*;

    use crate::simul::{Acceleration, Velocity};

    pub fn traverse(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity)>) {
        for (mut transform, velocity) in &mut bodies {
            let street = Vec2::from(*velocity) * time.delta_seconds();
            transform.translation += street.extend(0.);
        }
    }
    pub fn accelerate(time: Res<Time>, mut bodies: Query<(&mut Velocity, &Acceleration)>) {
        for (mut velocity, acceleration) in &mut bodies {
            let acceleration_inc = **acceleration * time.delta_seconds();
            **velocity += acceleration_inc;
        }
    }
}
