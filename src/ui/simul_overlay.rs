use bevy::prelude::*;

#[derive(Default)]
pub struct SimulOverlayPlugin {
    _priv_fields_placeholder: (),
}
impl Plugin for SimulOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys::spawn)
            .add_systems(Update, sys::update_passed_sect_count_disp);
    }
}

pub mod bundles;

pub mod compos {
    pub mod passed_sect_count {
        use bevy::prelude::*;

        #[derive(Default, Component)]
        pub struct PassedSectCountDisp {
            _priv_fields_placeholder: (),
        }
    }
}
pub mod sys {
    use bevy::prelude::*;

    use crate::ui::SimulOverlayBundle;
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(SimulOverlayBundle::default())
            .with_children(|child_builder| {
                child_builder.spawn(crate::ui::PassedSectCountDispBundle::new(&asset_server));
            });
    }

    pub fn update_passed_sect_count_disp(
        passed_sect_count: Res<crate::misc::PassedSectCount>,
        mut query: Query<&mut Text, With<crate::ui::PassedSectCountDisp>>,
    ) {
        if passed_sect_count.is_changed() {
            for mut text in query.iter_mut() {
                text.sections[1].value = passed_sect_count.to_string();
            }
        }
    }
}
