pub mod collide;

use bevy::prelude::*;

use crate::{
    simul::{self, hero, HeroColor, HeroCore, HeroHop, Velocity},
    SimulState,
};

pub fn spawn(mut cmds: Commands, color: Res<HeroColor>) {
    cmds.spawn(crate::simul::HeroBundle::with_color((*color).into()));
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
    simul_state: Res<State<SimulState>>,
) {
    if !simul_state.is_running() {
        return;
    }

    let Ok(mut transform) = hero.get_single_mut() else {
        // No hero, no hopping.
        return;
    };
    for _hop in hop_listener.read() {
        transform.translation.y += simul::hero::HOP_UP_HEIGHT;
    }
}

pub fn up_implies_downs(
    mut hero: Query<(&mut Velocity,), With<HeroCore>>,
    mut hop_listener: EventReader<HeroHop>,
    simul_state: Res<State<SimulState>>,
) {
    if !simul_state.is_running_without_gravity() {
        return;
    }
    if !hop_listener.is_empty() {
        if let Ok((mut hero_velocity,)) = hero.get_single_mut() {
            hero_velocity.y = hero::INIT_VELOCITY.y;
        }
        hop_listener.clear();
    }
}

pub fn update_displayed_color(
    mut hero: Query<(&mut Sprite,), With<crate::simul::HeroCore>>,
    logical_color: Res<HeroColor>,
) {
    if logical_color.is_changed() {
        for (mut hero_sprite,) in &mut hero {
            hero_sprite.color = (*logical_color).into();
        }
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
        transform.rotate(Quat::from_rotation_z(hero::HEAD_UP_ANGLE));
    }
}
