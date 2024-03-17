use bevy::prelude::*;

use crate::ui::MotionScaleDisp;

#[derive(Bundle)]
pub struct MotionScaleDispBundle {
    pub name: Name,
    pub motion_scale_disp: MotionScaleDisp,
    pub text: TextBundle,
}

impl MotionScaleDispBundle {
    pub const FONT_SIZE: f32 = 60.0;
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        MotionScaleDispBundle {
            name: Name::new("MotionScaleDisp"),
            motion_scale_disp: MotionScaleDisp::default(),
            text: TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "MS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Cousine/Cousine-Regular.ttf"),
                            font_size: Self::FONT_SIZE,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "?".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Cousine/Cousine-Bold.ttf"),
                            font_size: Self::FONT_SIZE,
                            color: Color::GOLD,
                        },
                    },
                ]),
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        }
    }
}
