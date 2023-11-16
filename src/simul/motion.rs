use bevy::prelude::*;

#[derive(Default)]
pub struct MotionPlugin {
    _future_priv_fields: (),
}
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<compos::Motion>();
    }
}

pub mod compos {
    use bevy::prelude::*;
    use derive_more::Constructor;
    #[derive(Component, Debug, Reflect, Constructor)]
    #[reflect(Component)]
    pub struct Motion {
        pub velocity: f32, // u/s
    }
    impl Default for Motion {
        fn default() -> Self {
            Self::new(crate::simul::hero::INIT_VELOCITY)
        }
    }
}
