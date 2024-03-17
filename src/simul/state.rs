use bevy::prelude::*;

use crate::SimulState;

#[derive(Debug, Default)]
pub struct SimulStatePlugin {
    _priv_fields_placeholder: (),
}

impl Plugin for SimulStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_state::<crate::SimulState>()
            .init_resource::<crate::SimulStartTime>()
            // Reset simulation start time.
            .add_systems(
                OnEnter(SimulState::RunningWithoutGravity),
                sys::reset_simul_start_time_uncond,
            )
            // Move using graph edges.
            .add_systems(Update, (sys::circulate, sys::restart_game));
    }
}

pub mod states;

pub mod res {
    use std::time;

    use bevy::prelude::*;
    use derive_more::{From, Into};

    #[derive(Resource, Default, Clone, Copy, From, PartialEq, Eq, PartialOrd, Ord, Hash, Into)]
    pub struct SimulStartTime(time::Duration);
}

pub mod sys {
    use bevy::prelude::*;

    use crate::{simul::HeroDeath, SimulState};

    use crate::SimulStartTime;

    pub fn restart_game(
        mut restart_cause: EventReader<HeroDeath>,
        mut next_state: ResMut<NextState<SimulState>>,
    ) {
        if !restart_cause.is_empty() {
            let _ = next_state.0.insert(SimulState::Cleanup);
            restart_cause.clear();
        }
    }

    pub fn circulate(state: Res<State<SimulState>>, mut next_state: ResMut<NextState<SimulState>>) {
        if !state.is_running() {
            let _ = *next_state.0.insert(state.forward());
        }
    }

    pub fn reset_simul_start_time_uncond(mut start_time: ResMut<SimulStartTime>, time: Res<Time>) {
        *start_time = time.elapsed().into();
    }
}
