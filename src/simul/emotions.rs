//! Encapsulates the logic of humoristic "emotions" that are in fact effects.
//!  "Emotions" can be put on in-game entities.

use bevy::prelude::*;

/// Enables affecting entities with emotion components.
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

/// Components.
pub mod compos {
    use bevy::prelude::*;

    /// The basic emotion that adventure-thirsty Flapik is bound to experience.
    ///
    /// It makes his a head go down. And the body starts heading towords the ground.
    /// This makes for the fall that generally causes death to the depressed birds.
    /// Hopefully player does something to mitigate this effect.
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
            // FixMe
            // victim.rotation.z -= self.effect_strength(victim) * time.delta_seconds();
        }
    }
}

/// Systems making emotions have actual impact on their victims.
pub mod sys {
    use bevy::prelude::*;
    /// System that makes sure that all bordom-possessing entities experience its cruelty.
    pub fn boredom(
        time: Res<Time>,
        mut victims: Query<(&mut Transform, &crate::simul::emotions::compos::Boredom)>,
    ) {
        for (mut transform, effect) in &mut victims {
            effect.apply_effect(&mut transform, &time);
        }
    }
}
