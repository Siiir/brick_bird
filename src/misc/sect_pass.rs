use bevy::prelude::*;

use crate::misc::PassedSectCount;

#[derive(Debug, Default)]
pub struct SectPassPlugin {
    _priv_fields_placeholder: (),
}
impl Plugin for SectPassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PassedSectCount>()
            .add_event::<PassedSectCount>()
            .add_systems(Update, sys::update_count);
    }
}

pub mod sys {
    use bevy::prelude::*;

    use crate::misc::PassedSectCount;

    pub fn update_count(
        mut count: ResMut<PassedSectCount>,
        mut event_reader: EventReader<PassedSectCount>,
    ) {
        for event in &mut event_reader {
            *count += *event;
        }
    }
}
pub mod res {
    pub mod count {
        use std::num::NonZeroU128;

        use bevy::prelude::*;
        use derive_more::{
            Add, AddAssign, Deref, DerefMut, Display, Div, DivAssign, Into, Mul, MulAssign, Sub,
            SubAssign,
        };

        #[derive(
            Debug,
            Display,
            Into,
            Copy,
            Clone,
            Event,
            Resource,
            Reflect,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Add,
            Sub,
            Mul,
            Div,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Hash,
            Deref,
            DerefMut,
        )]
        #[reflect(Resource)]
        pub struct PassedSectCount(u128);

        impl PassedSectCount {
            /// This is recommended way of constructing events to ensure that they are meaningful.
            pub fn new_event(num: NonZeroU128) -> Self {
                Self(num.get())
            }
            pub fn new_resource(num: u128) -> Self {
                Self(num)
            }
        }

        impl Default for PassedSectCount {
            fn default() -> Self {
                Self::new_resource(0)
            }
        }
    }
}
