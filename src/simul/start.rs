use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SimulStartPlugin {
    _priv_fields_placeholder: (),
}
impl Plugin for SimulStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SimulStart>()
            .add_systems(Startup, sys::announce);
    }
}

pub mod events {
    use bevy::prelude::*;

    #[derive(Debug, Event)]
    pub struct SimulStart {
        _priv_fields_placeholder: (),
    }

    impl SimulStart {
        pub fn new() -> Self {
            Self {
                _priv_fields_placeholder: (),
            }
        }
    }
}
pub mod sys {
    use bevy::prelude::*;

    pub fn announce(mut announcer: EventWriter<crate::SimulStart>) {
        announcer.send(crate::SimulStart::new());
    }
}
