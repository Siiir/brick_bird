use bevy::prelude::*;

pub fn advance(
    mut plane: ResMut<crate::SimulPlane>,
    hero: Query<(&Transform,), (With<crate::simul::HeroCore>,)>,
) {
    let Ok(hero) = hero.get_single() else {
        // No hero, no advances.
        return;
    };
}
