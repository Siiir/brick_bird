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
pub mod gravitation {
    use bevy::prelude::*;

    #[derive(Default, Clone, Copy, Component, Reflect, Debug)]
    #[reflect(Component)]
    pub struct Gravitation;
}
