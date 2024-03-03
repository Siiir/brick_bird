use crate::simul::{HeroCore, ObservatorBundle};
use bevy::{prelude::*, window::WindowResized};

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
    }
}

pub fn spawn(mut cmds: Commands) {
    cmds.spawn(crate::simul::ObservatorBundle::default());
}

pub fn maximize_win(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.set_maximized(true);
}

pub fn react_to_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut cameras: Query<(&mut Transform,), (With<Camera>,)>,
) {
    if let Some(last_win_size) = resize_events.read().last() {
        let win_size: Vec2 = [last_win_size.width, last_win_size.height].into();
        debug!("Last window resize: {win_size}");
        let (mut camera_transform,) = cameras.single_mut();

        let new_scale = ObservatorBundle::scale_from_win_size(win_size);
        camera_transform.scale.x = new_scale;
        camera_transform.scale.y = new_scale;
    }
}
