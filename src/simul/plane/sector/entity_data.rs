use bevy::prelude::*;
use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct EntityData {
    id: Entity,
    translation_x: f32,
}
impl EntityData {
    // Getters
    pub fn id(&self) -> Entity {
        self.id
    }
    pub fn translation_x(&self) -> f32 {
        self.translation_x
    }
    pub fn right_bound_x(&self) -> f32 {
        self.translation_x() + 0.5 * crate::simul::Sector::SCALE.x
    }
}
