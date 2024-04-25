//! Systems that make the plane objects usefull and impactful.

use bevy::{app::AppExit, prelude::*, window::WindowCloseRequested};
use std::num::NonZeroU128;

use crate::{misc::PassedSectCount, SimulPlane};

pub fn reset_logical_plane(mut plane: ResMut<SimulPlane>) {
    *plane = SimulPlane::default();
}

/// Automatically advances the auto-generated in-simulation plane.
///
/// This advancement of the state of the "plane object" boils down to
///  memory management and dynamic world generation.
/// Thanks to the dynamic nature of the "plane object", the user experiences the
///  in simulation plane as a continuous & infinite game world,
///  while the memory complexity of this infinite world remains constant.
pub fn advance(
    hero_color: Res<crate::simul::HeroColor>,
    mut plane: ResMut<crate::SimulPlane>,
    hero: Query<(&Transform,), (With<crate::simul::HeroCore>,)>,
    mut pass_event_writer: EventWriter<PassedSectCount>,
    mut cmds: Commands,
) {
    let Ok((hero_transform,)) = hero.get_single() else {
        // No hero, no advances.
        return;
    };
    let mut passed_sect_count = 0;
    loop {
        let current_hero_sector = plane.hero_sect();
        let curr_hero_sect_bound = current_hero_sector.right_bound_x();
        if hero_transform.translation.x <= curr_hero_sect_bound {
            // Everything ok :)
            break;
        };
        plane.advance(hero_color.rbg(), &mut rand::thread_rng(), &mut cmds);
        passed_sect_count += 1;
    }
    if let Some(meaningful_count) = NonZeroU128::new(passed_sect_count) {
        pass_event_writer.send(PassedSectCount::new_event(meaningful_count));
    }
}

pub fn run_special_drop_of_sects(
    cmds: Commands,
    mut app_exits: EventReader<AppExit>,
    mut win_close_reqs: EventReader<WindowCloseRequested>,
    simul_plane: ResMut<SimulPlane>,
) {
    if app_exits.is_empty() && win_close_reqs.is_empty() {
        return;
    }
    SimulPlane::despawn(simul_plane, cmds);
    app_exits.clear();
    win_close_reqs.clear();
}
