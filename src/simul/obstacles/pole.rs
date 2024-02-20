//! Encapsulates the logic of behind the `Pole`s.

use bevy::prelude::*;

use crate::simul::Sector;

/// A basic obstacle a hero can bump into.
///
/// It is generally unpassable and meant to just block hero's way.
/// It generally kills hero if they try to penetrate through it.
#[derive(Clone, Copy, Default, Debug)]
pub struct Pole {
    scale: Vec2,
}
impl Pole {
    // Constants – Given

    /// The standard width of the in-game poles.
    pub const STD_SCALE_X: core::ops::RangeInclusive<f32> = 0.2..=0.5;
    /// The display layer that is relative to the parent entity.
    pub const LOCAL_DISPLAY_LAYER: f32 = 1.;
    /// Marks the x-axis position of pole when rendered on its sector.
    pub const LOCAL_X: f32 = 0.;

    // CRUD-C: Constructors

    pub fn new(scale: impl Into<Vec2>) -> Self {
        Self {
            scale: scale.into(),
        }
    }

    // CRUD-R: Getters

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    // CRUD-R: Properties

    pub fn width(&self) -> f32 {
        self.scale.x * Sector::SCALE.x
    }
    pub fn height(&self) -> f32 {
        self.scale.y * Sector::SCALE.y
    }

    pub fn upper_bound_y(&self, self_lower_bound_y: f32) -> f32 {
        self_lower_bound_y + self.height()
    }
    pub fn lower_bound_y(&self, self_upper_bound_y: f32) -> f32 {
        self_upper_bound_y - self.height()
    }
    pub fn left_bound_x(&self, self_x_coordinate: f32) -> f32 {
        self_x_coordinate - self.width() / 2.
    }
    pub fn right_bound_x(&self, self_x_coordinate: f32) -> f32 {
        self_x_coordinate + self.width() / 2.
    }

    // CRUD-U: Updaters

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
                    scale: self.scale.extend(0.).into(),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
