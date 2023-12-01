//! Systems that make the plane objects usefull and impactful.

use bevy::prelude::*;

/// Automatically advances the auto-generated in-simulation plane.
///
/// This advancement of the state of the "plane object" boils down to
///  memory management and dynamic world generation.
/// Thanks to the dynamic nature of the "plane object", the user experiences the
///  in simulation plane as a continuous & infinite game world,
///  while the memory complexity of this infinite world remains constant.
pub fn advance(
    mut plane: ResMut<crate::SimulPlane>,
    hero: Query<(&Transform,), (With<crate::simul::HeroCore>,)>,
) {
    let Ok(hero) = hero.get_single() else {
        // No hero, no advances.
        return;
    };
    todo!()
}
