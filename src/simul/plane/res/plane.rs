//! Module encapsulating `SimulPlane` struct logic.

use std::{collections::VecDeque, iter, sync::atomic};

use crate::simul::{plane::sector, Sector};
use bevy::prelude::*;
use derive_more::IntoIterator;

/// The **interface** of the infinite in-game **traversable plane**.
///
/// It is **filled with obstacles** for hero to encounter.
/// It is also a **container for "plane sectors"**.
/// Facilitates the **location detection** through _sector detection system_.
#[derive(Resource, Debug, IntoIterator)]
pub struct SimulPlane {
    /// # Invariants:
    /// * Should have length = [`Self::SECT_COUNT`] .
    #[into_iterator(ref_mut, ref)]
    x_axis: std::collections::VecDeque<Sector>,
}

impl SimulPlane {
    // Constants – Given

    pub const HERO_SECT_IDX: usize = 3;
    const COUNT_OF_INITIALLY_EMPTY_SECTS_BEFORE_HERO: usize = 1;
    /// The default count of sectors in the plane.
    pub const SECT_COUNT: usize = 9;
    /// The x cordinate on which the first sector will be rendered.
    pub const DEFAULT_FIRST_SECT_X: f32 = 0.0;
    /// The minimum color contrast between neighbour sectors and poles.
    ///
    /// As determined be the `game::color::rbg_contrast` function.
    pub const MIN_SECT_COLOR_CONTRAST: f32 = 0.9;

    pub const CONTAINING_AT_LEAST_1_SECT_EXPECTATION: &'static str =
        "Simulation plane should contain at least 1 sector.";

    // Constants – Calculated
    pub const INITIALLY_EMPTY_SECTS_COUNT: usize =
        Self::HERO_SECT_IDX + 1 + Self::COUNT_OF_INITIALLY_EMPTY_SECTS_BEFORE_HERO;
    pub const INITIALLY_FILLED_SECTS_COUNT: usize =
        Self::SECT_COUNT - Self::INITIALLY_EMPTY_SECTS_COUNT;
    const NEXT_SECT_OFFSET: f32 = Sector::SCALE.x;

    // Assertions

    const _CONTAINS_AT_LEAST_1_SECTOR: () = assert!(
        0 < Self::SECT_COUNT,
        "{}",
        Self::CONTAINING_AT_LEAST_1_SECT_EXPECTATION
    );

    // CRUD-C: Constructors

    /// Creates an open, unobstructed simulation plane, which is really borring.
    pub fn new_open() -> Self {
        let mut next_sect_x = SimulPlane::DEFAULT_FIRST_SECT_X;
        SimulPlane {
            x_axis: iter::from_fn(|| Some(next_empty_sector(&mut next_sect_x)))
                .take(SimulPlane::SECT_COUNT)
                .collect(),
        }
    }

    // CRUD-R: Getters

    pub fn first_sect(&self) -> &Sector {
        self.x_axis
            .front()
            .expect(Self::CONTAINING_AT_LEAST_1_SECT_EXPECTATION)
    }
    /// Gets the plane sector on which the hero resides.
    pub fn hero_sect(&self) -> &Sector {
        &self.x_axis[Self::HERO_SECT_IDX]
    }
    pub fn last_sect(&self) -> &Sector {
        self.x_axis
            .back()
            .expect(Self::CONTAINING_AT_LEAST_1_SECT_EXPECTATION)
    }

    // CRUD-R: Properties

    /// Returns the x coordinate of the first sector in `self`.
    pub fn first_sect_x(&self) -> f32 {
        self.first_sect().translation_x()
    }
    pub fn last_sect_x(&self) -> f32 {
        self.last_sect().translation_x()
    }
    pub fn next_sect_x(&self) -> f32 {
        self.last_sect_x() + Self::NEXT_SECT_OFFSET
    }

    // CRUD-U: Initializers

    /// **Spawns the entities** representing already initialized sector objects.
    ///
    /// Makes the sector objects refer to the spawned in-simulation entities. Effectively making these objects useful.
    ///
    /// # Expectations
    /// This is expected to be called at the start of the simulation to create entities corresponding to the logical plane sectors.
    pub fn spawn_sects(
        mut plane: ResMut<SimulPlane>,
        hero_color: Res<crate::simul::HeroColor>,
        mut cmds: Commands,
    ) {
        for sector in &mut plane.x_axis {
            sector
                .spawn_with_rand_color(hero_color.rbg(), &mut cmds)
                .unwrap()
        }
    }

    // CRUD-U: Updaters

    /// Despawns the sector entities that the sector objects inside this plane refer to.
    ///
    /// This makes the contained sector objects refering to nothing. Effectivelly putting them to sleep.
    pub fn despawn_sects(mut this: ResMut<Self>, mut cmds: Commands) {
        for sector in &mut this {
            sector.despawn(&mut cmds).unwrap();
        }
    }
    /// Advances the logical plane. Generates new sectors. Drops the old ones.
    ///
    /// This method relyies on the simulation plane's sectors being already spawned.
    /// Use `.spawn_sects` to spawn them.
    pub fn advance<R: rand::Rng + ?Sized>(
        &mut self,
        hero_color_rbg: [f32; 3],
        rng: &mut R,
        cmds: &mut Commands,
    ) {
        // Add back
        let mut new_sect = Sector::rand_with_translation_x(rng, self.next_sect_x());
        new_sect.spawn_with_rand_color(           hero_color_rbg,
            cmds,
        ).expect("This new sector should not have a corresponding entity yet, because it has just been generated.");
        self.x_axis.push_back(new_sect);
        // Remove displayed front
        self.x_axis.pop_front().expect("Expected `self.x_axis` to be non-empty, because a value has been just added to it.").despawn(cmds).unwrap();
    }
}

// CRUD-C: Constructors

/// Serves as a good peaceful space for the player to start in before they start to encounter obstacles.
impl Default for SimulPlane {
    fn default() -> Self {
        let mut next_sect_x = SimulPlane::DEFAULT_FIRST_SECT_X;
        let mut x_axis: VecDeque<Sector> =
            iter::from_fn(|| Some(next_empty_sector(&mut next_sect_x)))
                .take(Self::INITIALLY_EMPTY_SECTS_COUNT)
                .collect();
        x_axis.extend(
            iter::from_fn(|| Some(next_rand_sector(&mut rand::thread_rng(), &mut next_sect_x)))
                .take(Self::INITIALLY_FILLED_SECTS_COUNT),
        );

        Self { x_axis }
    }
}

impl rand::distributions::Distribution<SimulPlane> for rand::distributions::Standard {
    fn sample<R>(&self, rng: &mut R) -> SimulPlane
    where
        R: rand::Rng + ?Sized,
    {
        let mut next_sect_x = SimulPlane::DEFAULT_FIRST_SECT_X;
        SimulPlane {
            x_axis: iter::from_fn(|| Some(next_rand_sector(rng, &mut next_sect_x)))
                .take(SimulPlane::SECT_COUNT)
                .collect(),
        }
    }
}

// CRUD-C: Fabrication functions

fn next_empty_sector(next_sect_x: &mut f32) -> Sector {
    let next_empty_sector = Sector::empty_with_translation_x(*next_sect_x);
    *next_sect_x += SimulPlane::NEXT_SECT_OFFSET;
    next_empty_sector
}
fn next_rand_sector<R>(rng: &mut R, next_sect_x: &mut f32) -> Sector
where
    R: rand::Rng + ?Sized,
{
    let next_empty_sector = Sector::rand_with_translation_x(rng, *next_sect_x);
    *next_sect_x += SimulPlane::NEXT_SECT_OFFSET;
    next_empty_sector
}

// CRUD-D: Destructor

impl Drop for SimulPlane {
    fn drop(&mut self) {
        if std::thread::panicking() {
            sector::DROP_ERR_HAS_BEEN_DISPLAYED.store(true, atomic::Ordering::SeqCst)
        }
    }
}
