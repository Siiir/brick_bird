pub mod collide;

use bevy::prelude::*;

use crate::simul::{self, HeroCore, HeroHop};

pub fn spawn(mut cmds: Commands) {
    cmds.spawn(crate::simul::HeroBundle::default());
}
pub fn despawn_if_present(mut cmds: Commands, hero: Query<Entity, With<crate::simul::HeroCore>>) {
    let Ok(hero) = hero.get_single() else {
        // no hero ==> nth to clean
        return;
    };
    cmds.entity(hero).despawn_recursive();
}

/// Makes hero experience a positive stimulation, whenever the players clicks space key.
pub fn hop(
    mut hero: Query<&mut Transform, With<crate::simul::HeroCore>>,
    mut hop_listener: EventReader<HeroHop>,
) {
    let Ok(mut transform) = hero.get_single_mut() else {
        // No hero, no hopping.
        return;
    };
    for _hop in hop_listener.read() {
        transform.translation.y += simul::hero::HOP_UP_HEIGHT;
    }
}

pub fn up_implies_downs(
    mut hero_velocity: Query<(&mut crate::simul::Velocity,), With<HeroCore>>,
    mut hop_listener: EventReader<HeroHop>,
) {
    if !hop_listener.is_empty() {
        if let Ok((mut hero_velocity,)) = hero_velocity.get_single_mut() {
            hero_velocity.y = -2. * crate::simul::hero::INIT_VELOCITY;
        }
        hop_listener.clear();
    }
}

#[deprecated]
/// Makes hero experience a positive stimulation, whenever the players clicks space key.
pub fn head_up_flappik(
    kbd_input: Res<ButtonInput<KeyCode>>,
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
