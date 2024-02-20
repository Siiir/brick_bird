use crate::simul::HeroCore;
use bevy::prelude::*;

/// Loose system that makes camera follow hero.
///
/// The looseness of this system means that it runs only when both hero & camera are available
///  and has no expectations about their availability.
pub fn follow_hero(
    mut cam: Query<(&mut Transform,), (With<Camera>, Without<HeroCore>)>,
    hero: Query<(&Transform,), (With<HeroCore>, Without<Camera>)>,
) {
    // Unwrapping
    let (Ok((mut cam,)), Ok((hero,))) = (cam.get_single_mut(), hero.get_single()) else {
        // Since it's loose implementation it idles when work conditions are inconvenient.
        return;
    };
    // Spatial operations.
    {
        let cam = &mut cam.translation;
        let hero = &hero.translation;
        cam.x = hero.x + crate::simul::Sector::SCALE.x;
        cam.y = hero.y;
    }
}
