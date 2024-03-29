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
        let hero_sect = &simul_plane.hero_sect();
        if (hero::upper_bound_y(hero_transform))
            > (hero_sect.upper_pole_lower_bound_y())
            && (hero::right_bound_x(hero_transform))
                > (hero_sect.upper_pole_left_bound_x())
        // And is not behind, because being behind is allowed (even with a touch)
            && !(hero::left_bound_x(hero_transform) >= hero_sect.upper_pole_right_bound_x())
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
        let hero_sect = &simul_plane.hero_sect();
        if hero::lower_bound_y(hero_transform) < hero_sect.lower_pole_upper_bound_y()
            && hero::right_bound_x(hero_transform) > hero_sect.lower_pole_left_bound_x()
        // And is not behind, because being behind is allowed (even with a touch)
        && !(hero::left_bound_x(hero_transform) >= hero_sect.lower_pole_right_bound_x())
        {
            event_writer.send(HeroDeath::default());
        }
    };
}
