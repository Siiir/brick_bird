use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulFinishPlugin {
    _priv_fields_placeholder: (),
}

impl Plugin for SimulFinishPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SimulFinish>();
    }
}

pub mod events {
    use bevy::prelude::*;

    #[derive(Debug, Event)]
    pub struct SimulFinish {
        _priv_fields_placeholder: (),
    }

    impl SimulFinish {
        pub fn new() -> Self {
            Self {
                _priv_fields_placeholder: (),
            }
        }
    }
}
