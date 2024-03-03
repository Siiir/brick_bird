use bevy::prelude::*;

use crate::SimulState;

#[derive(Default)]
pub struct KeybindsPlugin {
    _priv_fields_placeholder: (),
}
impl Plugin for KeybindsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys::hero_hop.run_if(SimulState::is_running_cond()));
    }
}

pub mod sys {
    use bevy::prelude::*;

    use crate::simul::HeroHop;

    pub fn hero_hop(
        kbd_input: Res<ButtonInput<KeyCode>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        touch_input: Res<Touches>,
        mut hop_announcer: EventWriter<HeroHop>,
    ) {
        for press in [
            kbd_input.just_pressed(KeyCode::Space),
            mouse_input.just_pressed(MouseButton::Left),
            touch_input.any_just_pressed(),
        ]
        .into_iter()
        {
            if press {
                hop_announcer.send(HeroHop::new());
            }
        }
    }
}
