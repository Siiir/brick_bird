use bevy::prelude::*;

#[derive(Default)]
pub struct ObservationPlugin {
    _future_priv_fields: (),
}

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn(CameraBundle::new(default()));
        })
        .add_systems(Update, sys::follow_hero);
    }
}

pub mod sys {
    use crate::simul::HeroCore;
    use bevy::prelude::*;

    /// Loose following system.
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
            cam.x = hero.x;
            cam.y = hero.y;
        }
    }
}

pub use bundles::CameraBundle;
pub mod bundles {
    use bevy::prelude::*;
    use derive_more::Constructor;

    #[derive(Bundle, Constructor)]
    pub struct CameraBundle {
        base: Camera2dBundle,
    }
}
