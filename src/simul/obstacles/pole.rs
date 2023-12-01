//! Encapsulates the logic of behind the `Pole`s.

use bevy::prelude::*;

/// A basic obstacle a hero can bump into.
///
/// It is generally unpassable and meant to just block hero's way.
/// It generally kills hero if they try to penetrate through it.
#[derive(Clone, Copy, Default, Debug)]
pub struct Pole {
    pub width: f32,
    pub height: f32,
}
impl Pole {
    // Given
    /// The standard width of the in-game poles.
    pub const STD_WIDTH: core::ops::RangeInclusive<f32> = 0.2..=0.5;
    /// The display layer that is relative to the parent entity.
    pub const LOCAL_DISPLAY_LAYER: f32 = 1.;
    /// Marks the x-axis position of pole when rendered on its sector.
    pub const LOCAL_X: f32 = 0.;
    /// Convenience method for calling `.spawn`.
    pub fn spawn_as_upper(&self, parent: &mut ChildBuilder, color: [f32; 3]) {
        self.spawn(parent, color, true)
    }
    /// Convenience method for calling `.spawn`.
    pub fn spawn_as_lower(&self, parent: &mut ChildBuilder, color: [f32; 3]) {
        self.spawn(parent, color, false)
    }
    /// Spawns a pole entity on the parent.
    pub fn spawn(&self, parent: &mut ChildBuilder, color: [f32; 3], is_upper: bool) {
        let (translation_y, anchor) = if is_upper {
            (0.5, bevy::sprite::Anchor::TopCenter)
        } else {
            (-0.5, bevy::sprite::Anchor::BottomCenter)
        };

        parent.spawn((
            Name::from("Pole"),
            SpriteBundle {
                sprite: Sprite {
                    color: color.into(),
                    anchor,
                    ..default()
                },
                transform: Transform {
                    translation: [Self::LOCAL_X, translation_y, Self::LOCAL_DISPLAY_LAYER].into(),
                    scale: [self.width, self.height, 0.1].into(),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
