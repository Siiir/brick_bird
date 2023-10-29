use bevy::prelude::*;

#[derive(Default)]
pub struct ObstaclesPlugin {
    _future_priv_fields: (),
}

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub mod bundles {
    use bevy::prelude::*;

    #[derive(Bundle)]
    pub struct Sector {}
}
