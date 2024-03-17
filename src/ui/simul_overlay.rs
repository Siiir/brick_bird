pub use bundles::lowest_row::LowestRowBundle;
pub mod bundles;

pub mod compos {
    pub mod passed_sect_count {
        use bevy::prelude::*;

        #[derive(Default, Component)]
        pub struct PassedSectCountDisp {
            _priv_fields_placeholder: (),
        }
    }
    pub mod motion_scale {
        use bevy::prelude::*;

        #[derive(Default, Component)]
        pub struct MotionScaleDisp {
            _priv_fields_placeholder: (),
        }
    }
}
pub mod sys {
    use bevy::prelude::*;

    use crate::ui::{MotionScaleDispBundle, PassedSectCountDispBundle, SimulOverlayBundle};

    use super::LowestRowBundle;
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(SimulOverlayBundle::default())
            .with_children(|child_builder| {
                child_builder
                    .spawn(LowestRowBundle::default())
                    .with_children(|child_builder| {
                        child_builder.spawn(PassedSectCountDispBundle::new(&asset_server));
                        child_builder.spawn(MotionScaleDispBundle::new(&asset_server));
                    });
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
    pub fn update_motion_scale_disp(
        motion_scale: Res<crate::simul::MotionScale>,
        mut query: Query<&mut Text, With<crate::ui::MotionScaleDisp>>,
    ) {
        if motion_scale.is_changed() {
            for mut text in query.iter_mut() {
                let ms_val_mut = &mut text.sections[1].value;
                ms_val_mut.clear();
                use std::fmt::Write;
                write!(ms_val_mut, "{:.1}", **motion_scale)
                    .expect("Writing to empty string should not fail on targeted devices.");
            }
        }
    }
}

use bevy::prelude::*;

#[derive(Default)]
pub struct SimulOverlayPlugin {
    _priv_fields_placeholder: (),
}
impl Plugin for SimulOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys::spawn).add_systems(
            Update,
            (
                sys::update_passed_sect_count_disp,
                sys::update_motion_scale_disp,
            ),
        );
    }
}
