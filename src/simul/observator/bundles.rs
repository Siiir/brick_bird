/// -
use bevy::prelude::*;
use derive_more::Constructor;

/// All components of the in-game camera entity.
#[derive(Bundle, Constructor)]
pub struct ObservatorBundle {
    base: Camera2dBundle,
}
impl ObservatorBundle {
    pub fn scale_from_height(height: f32) -> f32 {
        910. / height
    }
}
impl Default for ObservatorBundle {
    fn default() -> Self {
        Self {
            base: Camera2dBundle {
                transform: Transform {
                    scale: [std::f32::NAN, std::f32::NAN, 1.].into(),
                    ..default()
                },
                ..default()
            },
        }
    }
}
