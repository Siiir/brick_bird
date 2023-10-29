use bevy::prelude::*;

#[derive(Default)]
pub struct HeroPlugin {
    _future_priv_fields: (),
}

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn(HeroBundle::default());
        });
    }
}

pub use compos::HeroCore;
pub mod compos {
    use bevy::prelude::*;
    #[derive(Component, Default)]
    pub struct HeroCore {
        _future_priv_fields: (),
    }
}

pub use bundles::HeroBundle;
pub mod bundles {
    use bevy::prelude::*;
    use derive_more::Constructor;
    #[derive(Bundle, Constructor)]
    pub struct HeroBundle {
        base: SpriteBundle,
    }

    impl Default for HeroBundle {
        fn default() -> Self {
            Self::new(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE,
                    ..default()
                },
                transform: Transform::from_scale([50., 20., 0.].into()),
                ..default()
            })
        }
    }
}
