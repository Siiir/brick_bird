//! Module encapsulating logic around `Sector` struct.

use crate::color;
use bevy::prelude::*;
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    sync::atomic::AtomicBool,
};

pub use entity_data::EntityData;
pub mod entity_data;
pub mod err {
    pub use entity_not_spawned::EntityNotSpawned;
    pub mod entity_not_spawned {
        use derive_more::Constructor;

        #[derive(Debug, Constructor, thiserror::Error)]
        #[error("Entity corresponding to this `{}` hasn't been spawned.", std::any::type_name::<crate::simul::Sector>())]
        pub struct EntityNotSpawned {}
    }
    pub use entity_already_present::EntityAlreadySpawned;
    pub mod entity_already_present {
        use derive_more::Constructor;

        #[derive(Debug, Constructor, thiserror::Error)]
        #[error("Entity corresponding to this `{}` has already been spawned.", std::any::type_name::<crate::simul::Sector>())]
        pub struct EntityAlreadySpawned {}
    }
}

/// A division of the simulation plane.
///
/// Can & is meant to be bound to a concrete entity that represents it. Contains cool properties which are used for the sector based calculations like: location detection, collision detection.
#[derive(Default, Debug)]
pub struct Sector {
    /// The concrete entity to which this logical object is bound to.
    entity: Option<EntityData>,
    /// Obstacle restricting passage through the upper part of this sector.
    upper_pole: crate::simul::obstacles::Pole,
    /// Obstacle restricting passage through the lower part of this sector.
    lower_pole: crate::simul::obstacles::Pole,
}
impl Sector {
    // Constants – Given

    /// The scale of all spawned sectors.
    pub const SCALE: Vec3 = Vec3::new(300.0, 900.0, 0.1);
    /// The minimal gap between the 2 poles that restrict passage through this sector.
    ///
    /// It is encoded as a fraction of this sector's height.
    /// The positive (and high enough) value of this constant ensures that the hero can always pass through the gap and remain untouched by the obstacles.
    pub const MIN_GAP: f32 = 0.2;
    /// The y coordinate for all spawned sectors.
    pub const TRANSLATION_Y: f32 = 0.;
    pub const DISPLAY_LAYER: f32 = 0.;

    // Constants – Calculated

    /// The maximum fraction of height that can be occupied by obstacles.
    ///
    /// Because this value is < 1 (enough less then), hero can pass through every sector untouched.
    /// This constant faciliates the compution of quantity of space/passage that is free to be occupied by some obstacles.
    pub const MAX_OCCUPIED_HEIGHT: f32 = 1.0 - Self::MIN_GAP;

    pub const UPPER_BOUND_Y: f32 = Self::TRANSLATION_Y + Self::SCALE.y / 2.;
    pub const LOWER_BOUND_Y: f32 = Self::TRANSLATION_Y - Self::SCALE.y / 2.;

    // CRUD-R: Getters

    pub fn upper_pole(&self) -> &crate::simul::obstacles::Pole {
        &self.upper_pole
    }

    pub fn lower_pole(&self) -> &crate::simul::obstacles::Pole {
        &self.lower_pole
    }

    fn entity(&self) -> Result<&EntityData, err::EntityNotSpawned> {
        self.entity.as_ref().ok_or(err::EntityNotSpawned::new())
    }

    pub fn translation_x(&self) -> Result<f32, err::EntityNotSpawned> {
        self.entity().map(EntityData::translation_x)
    }

    // CRUD-R: Properties

    // Properties

    /// Returns true <==> this logical sector is bound to concrete entity.
    pub fn entity_present(&self) -> bool {
        self.entity.is_some()
    }

    pub fn right_bound_x(&self) -> Result<f32, err::EntityNotSpawned> {
        self.entity().map(|entity_data| entity_data.right_bound_x())
    }

    pub fn upper_pole_lower_bound_y(&self) -> f32 {
        self.upper_pole().lower_bound_y(Self::UPPER_BOUND_Y)
    }
    pub fn lower_pole_upper_bound_y(&self) -> f32 {
        self.lower_pole().upper_bound_y(Self::LOWER_BOUND_Y)
    }
    pub fn upper_pole_left_bound_x(&self) -> Result<f32, err::EntityNotSpawned> {
        Ok(self.upper_pole().left_bound_x(self.translation_x()?))
    }
    pub fn lower_pole_left_bound_x(&self) -> Result<f32, err::EntityNotSpawned> {
        Ok(self.lower_pole().left_bound_x(self.translation_x()?))
    }
    pub fn upper_pole_right_bound_x(&self) -> Result<f32, err::EntityNotSpawned> {
        Ok(self.upper_pole().right_bound_x(self.translation_x()?))
    }
    pub fn lower_pole_right_bound_x(&self) -> Result<f32, err::EntityNotSpawned> {
        Ok(self.lower_pole().right_bound_x(self.translation_x()?))
    }

    // CRUD-U: Updaters

    /// Spawns the corresponding concrete entity for this logical sector object.
    ///
    /// Binds `self` to spawned entity.
    pub fn spawn(
        &mut self,
        translation_x: f32,
        color_rbg: [f32; 3],
        cmds: &mut Commands,
    ) -> Result<(), err::EntityAlreadySpawned> {
        if self.entity_present() {
            Err(err::EntityAlreadySpawned::new())
        } else {
            let _ = self.entity.insert(EntityData::new(
                self.spawn_sector_entity(cmds, translation_x, color_rbg),
                translation_x,
            ));
            Ok(())
        }
    }

    /// Despawns the concrete entity `self` is bounded to. Unbinds `self`.
    pub fn despawn(&mut self, cmds: &mut Commands) -> Result<(), err::EntityNotSpawned> {
        if let Some(entity) = self.entity.take() {
            cmds.entity(entity.id()).despawn_recursive();
            Ok(())
        } else {
            Err(err::EntityNotSpawned::new())
        }
    }

    // CRUD-U: Aiding updaters.

    fn spawn_sector_entity(
        &self,
        cmds: &mut Commands<'_, '_>,
        translation_x: f32,
        color_rbg: [f32; 3],
    ) -> Entity {
        cmds.spawn((
            Name::from("Sector"),
            SpriteBundle {
                sprite: Sprite {
                    color: color_rbg.into(),
                    ..default()
                },
                transform: Transform {
                    translation: [translation_x, Self::TRANSLATION_Y, Self::DISPLAY_LAYER].into(),
                    scale: Self::SCALE,
                    rotation: default(),
                },
                ..default()
            },
        ))
        .with_children(|child_builder: &mut ChildBuilder| {
            self.upper_pole.spawn_as_upper(
                child_builder,
                color::contrasting_rand_rbg(color_rbg, crate::SimulPlane::MIN_SECT_COLOR_CONTRAST),
            );
            self.lower_pole.spawn_as_lower(
                child_builder,
                color::contrasting_rand_rbg(color_rbg, crate::SimulPlane::MIN_SECT_COLOR_CONTRAST),
            )
        })
        .id()
    }
}

// CRUD-C: Constructors

/// Standard distribution of sectors.
impl rand::distributions::Distribution<Sector> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sector {
        use crate::simul::obstacles::Pole;

        let lower_pole = Pole::new([
            rng.gen_range(Pole::STD_SCALE_X),
            rng.gen_range(0.0..=left_vertical_space(0.0)),
        ]);
        let upper_pole = Pole::new([
            rng.gen_range(Pole::STD_SCALE_X),
            rng.gen_range(0.0..=left_vertical_space(lower_pole.scale().y)),
        ]);
        Sector {
            entity: None,
            upper_pole,
            lower_pole,
        }
    }
}

// CRUD-R: Properties

// Calculations

/// States how much vertical space/passage is still free to be occupied by some obstacles.
pub fn left_vertical_space(taken_space: f32) -> f32 {
    Sector::MAX_OCCUPIED_HEIGHT - taken_space
}

// CRUD-D: Destructors

impl Drop for Sector {
    fn drop(&mut self) {
        if self.entity_present() {
            static ERR_HAS_BEEN_DISPLAYED: AtomicBool = AtomicBool::new(false);

            // If the error hasn't been started being displayed before entering this `if`.
            if !ERR_HAS_BEEN_DISPLAYED.swap(true, std::sync::atomic::Ordering::SeqCst) {
                // Prepare for displaying the error.
                let backtrace = Backtrace::capture();
                let backtrace_info = match backtrace.status() {
                    BacktraceStatus::Captured => format!("\n{backtrace}"),
                    BacktraceStatus::Disabled => format!(" Note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace."),
                    BacktraceStatus::Unsupported | _ => format!(" Note: Backtrace is unsupported on your platform."),
                };
                // Display the error.
                use std::any::type_name;
                warn!(
                    "Internal logic error: Instance of `{sector_t:}` dropped without despawning the corresponding `{entity_t:}`. This might cause misbehaviour or be just a memory leak. Note: This error won't be shown anymore untill the app is restarted.{backtrace_info}",
                    sector_t = type_name::<Self>(),
                    entity_t = type_name::<Entity>()
                )
            }
        }
    }
}
