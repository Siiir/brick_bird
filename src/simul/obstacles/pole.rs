use bevy::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Pole {
    pub width: f32,
    pub height: f32,
}
impl Pole {
    // Given
    pub const STD_WIDTH: core::ops::RangeInclusive<f32> = 0.2..=0.5;
    pub const LOCAL_DISPLAY_LAYER: f32 = 1.;
    pub const LOCAL_X: f32 = 0.;
    // Convenience method for calling `.spawn`
    pub fn spawn_as_upper(&self, parent: &mut ChildBuilder, color: [f32; 3]) {
        self.spawn(parent, color, true)
    }
    // Convenience method for calling `.spawn`
    pub fn spawn_as_lower(&self, parent: &mut ChildBuilder, color: [f32; 3]) {
        self.spawn(parent, color, false)
    }
    pub fn spawn(&self, parent: &mut ChildBuilder, color: [f32; 3], is_upper: bool) {
        let (translation_y, anchor) = if is_upper {
            (0.5, bevy::sprite::Anchor::TopCenter)
        } else {
            (-0.5, bevy::sprite::Anchor::BottomCenter)
        };

        parent.spawn((
            Name::from("Pole"),
            SpriteBundle {
                sprite: Sprite {
                    color: color.into(),
                    anchor,
                    ..default()
                },
                transform: Transform {
                    translation: [Self::LOCAL_X, translation_y, Self::LOCAL_DISPLAY_LAYER].into(),
                    scale: [self.width, self.height, 0.1].into(),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
