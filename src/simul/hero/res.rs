pub mod color {
    use bevy::prelude::*;

    #[derive(Resource, Debug, Clone, Copy, PartialEq)]
    pub struct HeroColor {
        rbg: [f32; 3],
    }
    impl HeroColor {
        pub fn new_ignoring_alpha(color: Color) -> Self {
            let [r, g, b, _] = color.as_rgba_f32();
            Self { rbg: [r, g, b] }
        }
        // CRUD-R: Properties
        pub fn rbg(&self) -> [f32; 3] {
            self.rbg
        }
    }
    // CRUD-C: Constructors
    impl Default for HeroColor {
        fn default() -> Self {
            Self::new_ignoring_alpha(Color::ORANGE)
        }
    }
    // CRUD-R: Converters
    impl From<HeroColor> for Color {
        fn from(hero_color: HeroColor) -> Self {
            Color::rgb_from_array(hero_color.rbg())
        }
    }
}
#[deprecated]
pub mod without_gravity {
    use std::time::Duration;

    use bevy::{prelude::*, time::Stopwatch};

    #[derive(Resource, Default, Clone, Debug)]
    pub struct TimeWithoutGravity {
        stopwatch: Stopwatch,
    }
    impl TimeWithoutGravity {
        pub fn tick(&mut self, delta: Duration) -> &Self {
            self.stopwatch.tick(delta);
            self
        }
        /// Terminates & tells how much time elapsed without gravitation.
        pub fn terminate(&mut self) -> Duration {
            self.stopwatch.pause();
            self.stopwatch.elapsed()
        }
        pub fn is_terminated(&self) -> bool {
            self.stopwatch.paused()
        }
    }
}
