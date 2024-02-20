use crate::{
    simul::{hero, HeroDeath, Sector},
    SimulPlane,
};
use bevy::prelude::*;

pub fn with_ceiling(
    hero: Query<(&Transform,), With<crate::simul::HeroCore>>,
    mut event_writer: EventWriter<HeroDeath>,
) {
    if let Ok((hero_transform,)) = hero.get_single() {
        if hero::upper_bound_y(hero_transform) > Sector::UPPER_BOUND_Y {
            event_writer.send(HeroDeath::default());
        }
    };
}

pub fn with_floor(
    hero: Query<(&Transform,), With<crate::simul::HeroCore>>,
    mut event_writer: EventWriter<HeroDeath>,
) {
    if let Ok((hero_transform,)) = hero.get_single() {
        if hero::lower_bound_y(hero_transform) < Sector::LOWER_BOUND_Y {
            event_writer.send(HeroDeath::default());
        }
    };
}

pub fn with_upper_pole(
    hero: Query<(&Transform,), With<crate::simul::HeroCore>>,
    simul_plane: Res<SimulPlane>,
    mut event_writer: EventWriter<HeroDeath>,
) {
    if let Ok((hero_transform,)) = hero.get_single() {
        if hero::upper_bound_y(hero_transform) > simul_plane.hero_sect().upper_pole_lower_bound_y()
            && hero::right_bound_x(hero_transform)
                > simul_plane.hero_sect().upper_pole_left_bound_x().unwrap()
        {
            event_writer.send(HeroDeath::default());
        }
    };
}

pub fn with_lower_pole(
    hero: Query<(&Transform,), With<crate::simul::HeroCore>>,
    simul_plane: Res<SimulPlane>,
    mut event_writer: EventWriter<HeroDeath>,
) {
    if let Ok((hero_transform,)) = hero.get_single() {
        if hero::lower_bound_y(hero_transform) < simul_plane.hero_sect().lower_pole_upper_bound_y()
            && hero::right_bound_x(hero_transform)
                > simul_plane.hero_sect().lower_pole_left_bound_x().unwrap()
        {
            event_writer.send(HeroDeath::default());
        }
    };
}
