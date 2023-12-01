//! Module encapsulating the logic related to the in-game hero, who is controled by a player.

use bevy::prelude::*;

/// Initial hero velocity.
///
/// The velocity value with witch the hero entity will start
pub const INIT_VELOCITY: f32 = 100.;
pub const HEAD_UP_ANGLE: f32 = 0.3;

/// Provides hero with his behaviour and effect on the environment.
#[derive(Default)]
pub struct HeroPlugin {
    _future_priv_fields: (),
}

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn(crate::simul::HeroBundle::default());
        })
        .add_systems(Update, (sys::fly_flappik_fly, sys::head_up_flappik));
        app.register_type::<compos::HeroCore>();
    }
}

/// Hero's components.
pub mod compos {
    use bevy::prelude::*;
    /// Provides the hero's unique core.
    ///
    /// This marker allow to easily discern hero from other entities.
    #[derive(Component, Reflect, Default)]
    #[reflect(Component)]
    pub struct HeroCore {
        _future_priv_fields: (),
    }
}

/// Bundles.
pub mod bundles {
    use bevy::prelude::*;
    use derive_more::Constructor;

    #[derive(Bundle, Constructor)]
    pub struct HeroBundle {
        name: Name,
        core: crate::simul::HeroCore,
        base: SpriteBundle,
        motion: crate::simul::Motion,
        emotion: crate::simul::emotions::compos::Boredom,
    }
    impl HeroBundle {
        pub const DISPLAY_LAYER: f32 = 10.0;
    }
    /// Creates a hero bundle that will set the hero on the starting point of the game.
    ///
    /// Hero created this way will have default stats and start-friendly behaviour.
    impl Default for HeroBundle {
        fn default() -> Self {
            Self::new(
                Name::from("Flappek"),
                crate::simul::HeroCore::default(),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::ORANGE,
                        ..default()
                    },
                    transform: Transform {
                        translation: [
                            crate::SimulPlane::FIRST_SECT_X + crate::simul::Sector::SCALE.x,
                            0.,
                            Self::DISPLAY_LAYER,
                        ]
                        .into(),
                        scale: [100., 50., 0.1].into(),
                        ..default()
                    },
                    ..default()
                },
                crate::simul::Motion::default(),
                crate::simul::emotions::compos::Boredom::default(),
            )
        }
    }
}

/// Systems controling hero and their effect on the surrounding environment.
pub mod sys {
    use bevy::prelude::*;

    /// Forces hero to involuntary fly forward untill they die.
    ///
    /// This can be seen as sad BUT makes a good selling point.
    pub fn fly_flappik_fly(
        time: Res<Time>,
        mut hero: Query<(&mut Transform, &crate::simul::Motion), (With<crate::simul::HeroCore>,)>,
    ) {
        let Ok((mut transform, motion)) = hero.get_single_mut() else {
            // The hero can be absent. Then he won't fly.
            return;
        };
        let street = motion.velocity * time.delta_seconds();
        let forward = transform.right();
        transform.translation += street * forward;
    }

    /// Makes hero experience a positive stimulation, whenever the players clicks space key.
    pub fn head_up_flappik(
        kbd_input: Res<Input<KeyCode>>,
        mut hero: Query<&mut Transform, With<crate::simul::HeroCore>>,
    ) {
        let Ok(mut transform) = hero.get_single_mut() else {
            // No hero, no head.
            return;
        };
        if kbd_input.just_pressed(KeyCode::Space) {
            // Adjust this angle to control the rotation amount
            transform.rotate(Quat::from_rotation_z(crate::simul::hero::HEAD_UP_ANGLE));
        }
    }
}
