pub mod gravity {
    use bevy::prelude::*;

    use derive_more::{Deref, DerefMut, From, Into};

    #[derive(
        Resource, Copy, Clone, From, Deref, DerefMut, PartialEq, PartialOrd, Reflect, Into,
    )]
    #[reflect(Resource)]
    pub struct Gravity(f32);

    impl Default for Gravity {
        fn default() -> Self {
            Self::from(-400.) // m/s²
        }
    }
}
pub mod scale {
    use bevy::prelude::*;
    use derive_more::{Constructor, Deref, DerefMut, From, Into};

    #[derive(
        Resource,
        Clone,
        Copy,
        Constructor,
        From,
        Debug,
        Reflect,
        PartialEq,
        PartialOrd,
        Deref,
        DerefMut,
        Into,
    )]
    #[reflect(Resource)]
    pub struct MotionScale(f32);

    impl Default for MotionScale {
        fn default() -> Self {
            (1.).into()
        }
    }
}
