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
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        }
    }
}
pub mod lowest_row {

    use bevy::prelude::*;

    #[derive(Bundle)]
    pub struct LowestRowBundle {
        name: Name,
        node: NodeBundle,
    }

    impl Default for LowestRowBundle {
        fn default() -> Self {
            Self {
                name: "LowestRow".into(),
                node: NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(20.),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        }
    }
}
pub mod motion_scale_disp;
pub mod passed_sect_count_disp;
