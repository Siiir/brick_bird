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
    pub mod hop {
        use bevy::prelude::*;

        #[derive(Event, Debug)]
        pub struct HeroHop {
            _hight: (),
        }
        impl HeroHop {
            pub fn new() -> Self {
                Self { _hight: () }
            }
        }
    }
}

/// Bundles.
pub mod bundles;

// Resources
pub mod res;

/// Systems controling hero and their effect on the surrounding environment.
pub mod sys;

use bevy::prelude::*;

use crate::SimulState;

// Constants

/// Initial hero velocity.
///
/// The velocity value with witch the hero entity will start
pub const INIT_VELOCITY: f32 = 100.;
pub const HOP_UP_HEIGHT: f32 = 60.;
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
        app.init_resource::<crate::simul::HeroColor>()
            .add_event::<crate::simul::HeroDeath>()
            .add_event::<crate::simul::HeroHop>();

        use self::sys::collide;
        app
            // spawn systems
            .add_systems(OnEnter(SimulState::Startup), sys::spawn)
            // despawn sytems
            .add_systems(OnEnter(SimulState::Cleanup), sys::despawn_if_present)
            // movement systems
            .add_systems(
                Update,
                (sys::hop, sys::up_implies_downs).run_if(in_state(SimulState::Running)),
            )
            // collision systems
            .add_systems(
                Update,
                (
                    collide::with_ceiling,
                    collide::with_floor,
                    collide::with_lower_pole,
                    collide::with_upper_pole,
                )
                    .run_if(in_state(SimulState::Running)),
            )
            // Misc.
            .add_systems(Update, (sys::update_displayed_color,));
    }
}

// CRUD-R: Properties

fn upper_bound_y(hero_transform: &Transform) -> f32 {
    hero_transform.translation.y + hero_transform.scale.y / 2.
}
fn lower_bound_y(hero_transform: &Transform) -> f32 {
    hero_transform.translation.y - hero_transform.scale.y / 2.
}
fn left_bound_x(hero_transform: &Transform) -> f32 {
    hero_transform.translation.x - hero_transform.scale.x / 2.
}
fn right_bound_x(hero_transform: &Transform) -> f32 {
    hero_transform.translation.x + hero_transform.scale.x / 2.
}
