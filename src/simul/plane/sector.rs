//! Module encapsulating logic around `Sector` struct.

use bevy::prelude::*;

pub use entity_data::EntityData;
pub mod entity_data;

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
    // Given

    /// The scale of all spawned sectors.
    pub const SCALE: Vec3 = Vec3::new(300.0, 900.0, 0.1);
    /// The minimal gap between the 2 poles that restrict passage through this sector.
    ///
    /// It is encoded as a fraction of this sector's height.
    /// The positive (and high enough) value of this constant ensures that the hero can always pass through the gap and remain untouched by the obstacles.
    pub const MIN_GAP: f32 = 0.2;
    /// The y coordinate for all spawned sectors.
    const TRANSLATION_Y: f32 = 0.;
    pub const DISPLAY_LAYER: f32 = 0.;

    // Calculated

    /// The maximum fraction of height that can be occupied by obstacles.
    ///
    /// Because this value is < 1 (enough less then), hero can pass through every sector untouched.
    /// This constant faciliates the compution of quantity of space/passage that is free to be occupied by some obstacles.
    pub const MAX_OCCUPIED_HEIGHT: f32 = 1.0 - Self::MIN_GAP;

    // Calculations

    /// States how much vertical space/passage is still free to be occupied by some obstacles.
    pub fn left_vertical_space(taken_space: f32) -> f32 {
        Self::MAX_OCCUPIED_HEIGHT - taken_space
    }

    // Properties

    /// Returns true <==> this logical sector is bound to concrete entity.
    pub fn entity_present(&self) -> bool {
        self.entity.is_some()
    }

    // (De)Binding with entity

    /// Spawns the corresponding concrete entity for this logical sector object.
    ///
    /// Binds `self` to spawned entity.
    pub fn spawn(&mut self, translation_x: f32, color_rbg: [f32; 3], cmds: &mut Commands) {
        if self.entity_present() {
            panic!("Attempt to spawn simulation plane sector that has already been spawned.")
        }
        let _ = self.entity.insert(EntityData::new(
            self.spawn_sector_entity(cmds, translation_x, color_rbg),
            translation_x,
        ));
    }

    /// Despawns the concrete entity `self` is bounded to. Unbinds `self`.
    pub fn despawn(&mut self, cmds: &mut Commands) {
        let entity = self.entity.take().unwrap_or_else(|| {
            panic!("Attempt to despawn a simulation plane sector entity that has't been spawned.")
        });
        cmds.entity(entity.id()).despawn_recursive();
    }

    // Aid functions.

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
                game::color::contrasting_rand_rbg(
                    color_rbg,
                    crate::SimulPlane::MIN_SECT_COLOR_CONTRAST,
                ),
            );
            self.lower_pole.spawn_as_lower(
                child_builder,
                game::color::contrasting_rand_rbg(
                    color_rbg,
                    crate::SimulPlane::MIN_SECT_COLOR_CONTRAST,
                ),
            )
        })
        .id()
    }
}

// Standard distribution of sectors.
impl rand::distributions::Distribution<Sector> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sector {
        use crate::simul::obstacles::Pole;

        let lower_pole = Pole {
            width: rng.gen_range(Pole::STD_WIDTH),
            height: rng.gen_range(0.0..=Sector::left_vertical_space(0.0)),
        };
        let upper_pole = Pole {
            width: rng.gen_range(Pole::STD_WIDTH),
            height: rng.gen_range(0.0..=Sector::left_vertical_space(lower_pole.height)),
        };
        Sector {
            entity: None,
            upper_pole,
            lower_pole,
        }
    }
}
