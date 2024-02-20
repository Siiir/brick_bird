use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulStatePlugin {
    _priv_fields_placeholder: (),
}

impl Plugin for SimulStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<crate::SimulState>()
            .add_systems(Update, sys::circulate);
    }
}

pub mod states {
    use bevy::prelude::*;
    use num_enum::{FromPrimitive, IntoPrimitive};

    #[derive(
        Debug, Clone, Copy, Default, States, Eq, PartialEq, Hash, IntoPrimitive, FromPrimitive,
    )]
    #[repr(u8)]
    pub enum SimulState {
        #[default]
        Startup,
        StartupEnd,
        Running,
        Cleanup,
    }

    impl SimulState {
        pub fn forward(self) -> Self {
            (u8::from(self).wrapping_add(1)).into()
        }
        pub fn backward(self) -> Self {
            (u8::from(self).wrapping_sub(1)).into()
        }
    }
}

pub mod sys {
    use bevy::prelude::*;

    use crate::SimulState;

    pub fn circulate(state: Res<State<SimulState>>, mut next_state: ResMut<NextState<SimulState>>) {
        if **state != SimulState::Running {
            let _ = *next_state.0.insert(state.forward());
        }
    }
}
