//! Module encapsulating `SimulPlane` struct logic.

use crate::color;
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
    /// * Should have length > [`Self::HERO_SECT_IDX`] .
    #[into_iterator(owned, ref_mut, ref)]
    x_axis: std::collections::VecDeque<crate::simul::Sector>,
    /// The x coordinate of the first sector in `self`.
    first_sect_x: f32,
    /// The x coordinate of the next sector in `self` to be spawned.
    next_sect_x: f32,
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

    // Constants – Calculated
    pub const INITIALLY_EMPTY_SECTS_COUNT: usize =
        Self::HERO_SECT_IDX + 1 + Self::COUNT_OF_INITIALLY_EMPTY_SECTS_BEFORE_HERO;
    pub const INITIALLY_FILLED_SECTS_COUNT: usize =
        Self::SECT_COUNT - Self::INITIALLY_EMPTY_SECTS_COUNT;
    const NEXT_SECT_OFFSET: f32 = crate::simul::Sector::SCALE.x;

    // Assertions

    /// Makes sure that the default sector count is compatible with index.
    ///
    /// Meaning: it **won't render** hero index **out of bounds**.
    const _HERO_IDX_AND_SEC_COUNT: () =
        assert!(Self::INITIALLY_EMPTY_SECTS_COUNT < Self::SECT_COUNT);

    // CRUD-C: Constructors

    /// Creates an open, unobstructed simulation plane, which is really borring.
    pub fn empty() -> Self {
        Self {
            x_axis: std::iter::from_fn(|| Some(crate::simul::Sector::default()))
                .take(Self::SECT_COUNT)
                .collect(),
            first_sect_x: Self::DEFAULT_FIRST_SECT_X,
            next_sect_x: Self::DEFAULT_FIRST_SECT_X,
        }
    }

    // CRUD-C: Initializers

    /// **Spawns the entities** representing already initialized sector objects.
    ///
    /// Makes the sector objects refer to the spawned in-simulation entities. Effectively making these objects useful.
    ///
    /// # Expectations
    /// This is expected to be called at the start of the simulation to create entities corresponding to the logical plane sectors.
    pub fn spawn_sects(
        mut this: ResMut<SimulPlane>,
        hero_color: Res<crate::simul::HeroColor>,
        mut cmds: Commands,
    ) {
        let SimulPlane {
            x_axis,
            first_sect_x: _,
            next_sect_x,
        } = &mut *this;
        trace!("Spawned sects.");
        for sector in x_axis {
            Self::spawn_next_sect(hero_color.rbg(), sector, next_sect_x, &mut cmds).unwrap()
        }
    }

    // CRUD-R: Getters

    /// Gets the plane sector on which the hero resides.
    pub fn hero_sect(&self) -> &crate::simul::Sector {
        &self.x_axis[Self::HERO_SECT_IDX]
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
        let mut new_sect: crate::simul::Sector = rng.gen();
        Self::spawn_next_sect(
            hero_color_rbg,
            &mut new_sect,
            &mut self.next_sect_x,
            cmds,
        ).expect("This new sector should not have a corresponding entity yet, because it has just been generated.");
        self.x_axis.push_back(new_sect);
        // Move logical front
        self.first_sect_x += Self::NEXT_SECT_OFFSET;
        // Remove displayed front
        self.x_axis.pop_front().expect("Expected `self.x_axis` to be non-empty, because a value has been just added to it.").despawn(cmds).unwrap();
    }
    /// Creates and spawns the next sector that comes after the last one.
    ///
    /// Spawns new sector on the right of the simulation plane.
    fn spawn_next_sect(
        hero_color_rbg: [f32; 3],
        next_sect: &mut crate::simul::Sector,
        next_sect_x: &mut f32,
        cmds: &mut Commands,
    ) -> Result<(), crate::simul::plane::sector::err::EntityAlreadySpawned> {
        let next_sect_color_rbg =
            color::rand_rbg_contrasting(hero_color_rbg, SimulPlane::MIN_SECT_COLOR_CONTRAST);
        next_sect.spawn(*next_sect_x, next_sect_color_rbg.into(), cmds)?;
        *next_sect_x += Self::NEXT_SECT_OFFSET;
        Ok(())
    }
}

// CRUD-C: Constructors

// Trait based constructors

/// Serves as a good peaceful space for the player to start in before they start to encounter obstacles.
impl Default for SimulPlane {
    fn default() -> Self {
        use rand::Rng;

        Self {
            x_axis: std::iter::from_fn(|| Some(crate::simul::Sector::default()))
                .take(Self::INITIALLY_EMPTY_SECTS_COUNT)
                .chain(
                    rand::thread_rng()
                        .sample_iter(rand::distributions::Standard)
                        .take(Self::INITIALLY_FILLED_SECTS_COUNT),
                )
                .collect(),
            first_sect_x: Self::DEFAULT_FIRST_SECT_X,
            next_sect_x: Self::DEFAULT_FIRST_SECT_X,
        }
    }
}

impl rand::distributions::Distribution<SimulPlane> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SimulPlane {
        use rand::distributions;

        SimulPlane {
            x_axis: std::iter::from_fn(|| Some(distributions::Standard.sample(rng)))
                .take(SimulPlane::SECT_COUNT)
                .collect(),
            first_sect_x: SimulPlane::DEFAULT_FIRST_SECT_X,
            next_sect_x: SimulPlane::DEFAULT_FIRST_SECT_X,
        }
    }
}
