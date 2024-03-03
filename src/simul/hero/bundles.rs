use bevy::prelude::*;
use derive_more::Constructor;

use crate::simul::hero;

#[derive(Bundle, Constructor)]
pub struct HeroBundle {
    name: Name,
    core: crate::simul::HeroCore,
    base: SpriteBundle,
    velocity: crate::simul::Velocity,
}
impl HeroBundle {
    pub const DISPLAY_LAYER: f32 = 10.0;
    pub fn with_color(color: Color) -> Self {
        Self::new(
            Name::from("Flappek"),
            crate::simul::HeroCore::default(),
            SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    translation: [
                        crate::SimulPlane::DEFAULT_FIRST_SECT_X
                            + (crate::SimulPlane::HERO_SECT_IDX as f32)
                                * crate::simul::Sector::SCALE.x,
                        0.,
                        Self::DISPLAY_LAYER,
                    ]
                    .into(),
                    scale: [50., 25., 0.1].into(),
                    ..default()
                },
                ..default()
            },
            [hero::INIT_VELOCITY.x, 0.].into(),
        )
    }
}
