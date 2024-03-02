use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct HeroColor {
    rbg: [f32; 3],
}
impl HeroColor {
    pub fn new_ignoring_alpha(color: Color) -> Self {
        let [r, g, b, _] = color.as_rgba_f32();
        Self { rbg: [r, g, b] }
    }
    // CRUD-R: Properties
    pub fn rbg(&self) -> [f32; 3] {
        self.rbg
    }
}
// CRUD-C: Constructors
impl Default for HeroColor {
    fn default() -> Self {
        Self::new_ignoring_alpha(Color::ORANGE)
    }
}
// CRUD-R: Converters
impl From<HeroColor> for Color {
    fn from(hero_color: HeroColor) -> Self {
        Color::rgb_from_array(hero_color.rbg())
    }
}
