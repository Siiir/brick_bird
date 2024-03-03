pub mod node {
    use bevy::prelude::*;

    #[derive(Bundle)]
    pub struct SimulOverlayBundle {
        name: Name,
        node: NodeBundle,
    }

    impl Default for SimulOverlayBundle {
        fn default() -> Self {
            Self {
                name: "SimulOverlay".into(),
                node: NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        }
    }
}
pub mod passed_sect_count_disp {
    use bevy::prelude::*;

    use crate::ui::PassedSectCountDisp;

    #[derive(Bundle)]
    pub struct PassedSectCountDispBundle {
        pub name: Name,
        pub passed_sect_count_disp: PassedSectCountDisp,
        pub text: TextBundle,
    }

    impl PassedSectCountDispBundle {
        pub const FONT_SIZE: f32 = 60.0;
        pub fn new(asset_server: &Res<AssetServer>) -> Self {
            PassedSectCountDispBundle {
                name: Name::new("PassedSectCountDisp"),
                passed_sect_count_disp: PassedSectCountDisp::default(),
                text: TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "PS: ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: Self::FONT_SIZE,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "?".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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
}
