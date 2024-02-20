pub mod collide;

use bevy::prelude::*;

pub fn spawn(mut cmds: Commands) {
    cmds.spawn(crate::simul::HeroBundle::default());
}

/// Forces hero to involuntary fly forward untill they die.
///
/// This can be seen as sad BUT makes a good selling point.
pub fn fly_flappik_fly(
    time: Res<Time>,
    mut hero: Query<(&mut Transform, &crate::simul::Motion), (With<crate::simul::HeroCore>,)>,
) {
    let Ok((mut transform, motion)) = hero.get_single_mut() else {
        // The hero can be absent. Then he won't fly.
        return;
    };
    let street = motion.velocity * time.delta_seconds();
    let forward = transform.right();
    transform.translation += street * forward;
}

/// Makes hero experience a positive stimulation, whenever the players clicks space key.
pub fn head_up_flappik(
    kbd_input: Res<Input<KeyCode>>,
    mut hero: Query<&mut Transform, With<crate::simul::HeroCore>>,
) {
    let Ok(mut transform) = hero.get_single_mut() else {
        // No hero, no head.
        return;
    };
    if kbd_input.just_pressed(KeyCode::Space) {
        // Adjust this angle to control the rotation amount
        transform.rotate(Quat::from_rotation_z(crate::simul::hero::HEAD_UP_ANGLE));
    }
}
