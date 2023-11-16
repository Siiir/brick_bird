use bevy::prelude::*;

pub use entity_data::EntityData;
pub mod entity_data;

#[derive(Default, Debug)]
pub struct Sector {
    entity: Option<EntityData>,
    upper_pole: crate::simul::obstacles::Pole,
    lower_pole: crate::simul::obstacles::Pole,
}
impl Sector {
    // Given
    pub const SCALE: Vec3 = Vec3::new(300.0, 900.0, 0.1);
    pub const MIN_GAP: f32 = 0.2;
    pub const TRANSLATION_Y: f32 = 0.;
    pub const DISPLAY_LAYER: f32 = 0.;
    // Calculated
    pub const MAX_OCCUPIED_HEIGHT: f32 = 1.0 - Self::MIN_GAP;
    // Calculations
    pub fn left_vertical_space(taken_space: f32) -> f32 {
        Self::MAX_OCCUPIED_HEIGHT - taken_space
    }
    // Relation with entity
    pub fn entity_present(&self) -> bool {
        self.entity.is_some()
    }
    pub fn spawn(&mut self, translation_x: f32, color_rbg: [f32; 3], cmds: &mut Commands) {
        if self.entity_present() {
            panic!("Attempt to spawn simulation plane sector that has already been spawned.")
        }
        let _ = self.entity.insert(EntityData::new(
            self.spawn_sector_entity(cmds, translation_x, color_rbg),
            translation_x,
        ));
    }
    // The binding with entity
    #[deprecated(note = "Unmantained. Use `.spawn` instead.")]
    #[allow(warnings)]
    fn spawn_if_absent(&mut self, cmds: &mut Commands) {
        self.entity.get_or_insert_with(|| todo!("Spawn"));
    }
    pub fn despawn(&mut self, cmds: &mut Commands) {
        let entity = self.entity.take().unwrap_or_else(|| {
            panic!("Attempt to despawn a simulation plane sector entity that has't been spawned.")
        });
        cmds.entity(entity.id()).despawn_recursive();
    }
    // Helpers
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
                super::next_rand_color_rbg(color_rbg, crate::SimulPlane::MIN_SECT_COLOR_CONTRAST),
            );
            self.lower_pole.spawn_as_lower(
                child_builder,
                super::next_rand_color_rbg(color_rbg, crate::SimulPlane::MIN_SECT_COLOR_CONTRAST),
            )
        })
        .id()
    }
}

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
