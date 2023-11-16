use bevy::prelude::*;

#[derive(Default)]
pub struct EmotionsPlugin {
    _future_priv_fields: (),
}
impl Plugin for EmotionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (sys::boredom,));
        app.register_type::<compos::Boredom>();
    }
}

pub mod compos {
    use bevy::prelude::*;

    #[derive(Component, Debug, Default, Reflect)]
    #[reflect(Component)]
    pub struct Boredom {
        _future_priv_fields: (),
    }
    impl Boredom {
        pub fn effect_strength(&self, victim: &Transform) -> f32 {
            let _additional_coeff = self;
            let ret = (victim.translation.z + std::f32::consts::FRAC_PI_2).sqrt() - 0.2; // Epsilon
            if !(ret > 0.) {
                panic!("{ret:?}");
            }
            ret
        }
        pub fn apply_effect(&self, victim: &mut Transform, time: &Time) {
            victim.rotation.z -= self.effect_strength(victim) * time.delta_seconds();
        }
    }
}

pub mod sys {
    use bevy::prelude::*;
    pub fn boredom(
        time: Res<Time>,
        mut victims: Query<(&mut Transform, &crate::simul::emotions::compos::Boredom)>,
    ) {
        for (mut transform, effect) in &mut victims {
            effect.apply_effect(&mut transform, &time);
        }
    }
}
