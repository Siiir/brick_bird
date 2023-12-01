use bevy::prelude::*;
use derive_more::IntoIterator;

#[derive(Resource, Debug, IntoIterator)]
pub struct SimulPlane {
    #[into_iterator(owned, ref_mut, ref)]
    x_axis: std::collections::VecDeque<crate::simul::Sector>,
    next_sect_x: f32,
    next_sect_color_rbg: [f32; 3],
}

impl SimulPlane {
    // Given
    pub const HERO_SECT_IDX: usize = 1;
    pub const DEFAULT_SECT_COUNT: usize = 7;
    pub const FIRST_SECT_X: f32 = -650.0;
    pub const MIN_SECT_COLOR_CONTRAST: f32 = 0.5;
    // Assertions
    pub const _HERO_IDX_AND_SEC_COUNT: () = assert!(Self::HERO_SECT_IDX < Self::DEFAULT_SECT_COUNT);

    // Getters
    pub fn hero_sect(&self) -> &crate::simul::Sector {
        &self.x_axis[Self::HERO_SECT_IDX]
    }

    /// This is expected to be called at the start of the simulation to create entities corresponding to the logical plane sectors.
    pub fn spawn_sects(mut this: ResMut<Self>, mut cmds: Commands) {
        let SimulPlane {
            x_axis,
            next_sect_x,
            next_sect_color_rbg,
        } = &mut *this;
        for sector in x_axis {
            Self::spawn_next(sector, next_sect_x, next_sect_color_rbg, &mut cmds)
        }
    }
    pub fn despawn_sects(mut this: ResMut<Self>, mut cmds: Commands) {
        for sector in &mut this {
            sector.despawn(&mut cmds);
        }
    }
    /// This method relyies on the simulation plane being already spawned.
    pub fn advance<R: rand::Rng + ?Sized>(&mut self, rng: &mut R, cmds: &mut Commands) {
        // Add back
        let mut new_sect: crate::simul::Sector = rng.gen();
        Self::spawn_next(
            &mut new_sect,
            &mut self.next_sect_x,
            &mut self.next_sect_color_rbg,
            cmds,
        );
        self.x_axis.push_back(new_sect);
        // Remove front
        self.x_axis.pop_front().expect("Expected `self.x_axis` to be non-empty, because a value has been just added to it.").despawn(cmds);
    }
    fn spawn_next(
        next_sect: &mut crate::simul::Sector,
        next_sect_x: &mut f32,
        next_sect_color_rbg: &mut [f32; 3],
        cmds: &mut Commands,
    ) {
        next_sect.spawn(*next_sect_x, (*next_sect_color_rbg).into(), cmds);
        *next_sect_x += crate::simul::Sector::SCALE.x;
        *next_sect_color_rbg = game::color::contrasting_rand_rbg(
            *next_sect_color_rbg,
            SimulPlane::MIN_SECT_COLOR_CONTRAST,
        );
    }
}

// Trait based constructors

impl Default for SimulPlane {
    fn default() -> Self {
        Self {
            x_axis: std::iter::from_fn(|| Some(crate::simul::Sector::default()))
                .take(Self::DEFAULT_SECT_COUNT)
                .collect(),
            next_sect_x: Self::FIRST_SECT_X,
            next_sect_color_rbg: game::color::rand_rbg(),
        }
    }
}

impl rand::distributions::Distribution<SimulPlane> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SimulPlane {
        use rand::distributions;

        SimulPlane {
            x_axis: std::iter::from_fn(|| Some(distributions::Standard.sample(rng)))
                .take(SimulPlane::DEFAULT_SECT_COUNT)
                .collect(),
            next_sect_x: SimulPlane::FIRST_SECT_X,
            next_sect_color_rbg: game::color::rand_rbg(),
        }
    }
}
