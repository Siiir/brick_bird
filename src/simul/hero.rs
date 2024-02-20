//! Module encapsulating the logic related to the in-game hero, who is controled by a player.

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

pub mod events {
    pub mod death {
        use bevy::prelude::*;

        #[derive(Event, Debug, Default)]
        pub struct HeroDeath {
            _cause: (),
        }
    }
}

/// Bundles.
pub mod bundles;

/// Systems controling hero and their effect on the surrounding environment.
pub mod sys;

use bevy::prelude::*;

// Constants

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

// CRUD-C: Fabrication methods

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<compos::HeroCore>();
        app.add_event::<crate::simul::HeroDeath>();

        use self::sys::collide;
        app.add_systems(OnEnter(crate::SimulState::Startup), sys::spawn)
            .add_systems(
                Update,
                (sys::fly_flappik_fly, sys::head_up_flappik)
                    .run_if(in_state(crate::SimulState::Running)),
            )
            // add collision systems
            .add_systems(
                Update,
                (
                    collide::with_ceiling,
                    collide::with_floor,
                    collide::with_lower_pole,
                    collide::with_upper_pole,
                )
                    .run_if(in_state(crate::SimulState::Running)),
            );
    }
}

// CRUD-R: Properties

fn upper_bound_y(hero_transform: &Transform) -> f32 {
    hero_transform.translation.y + hero_transform.scale.y / 2.
}
fn lower_bound_y(hero_transform: &Transform) -> f32 {
    hero_transform.translation.y - hero_transform.scale.y / 2.
}
fn right_bound_x(hero_transform: &Transform) -> f32 {
    hero_transform.translation.x + hero_transform.scale.x / 2.
}
