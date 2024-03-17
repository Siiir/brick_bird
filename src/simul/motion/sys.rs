use std::time;

use bevy::prelude::*;

use crate::{
    simul::{Acceleration, Gravitation, Gravity, MotionScale, Velocity},
    SimulStartTime,
};

pub fn traverse(
    mut bodies: Query<(&mut Transform, &Velocity)>,
    motion_scale: Res<MotionScale>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut bodies {
        let street = Vec2::from(*velocity) * **motion_scale * time.delta_seconds();
        transform.translation += street.extend(0.);
    }
}
pub fn accelerate(
    mut bodies: Query<(&mut Velocity, &Acceleration)>,
    motion_scale: Res<MotionScale>,
    time: Res<Time>,
) {
    for (mut velocity, acceleration) in &mut bodies {
        let velocity_inc = **acceleration * **motion_scale * time.delta_seconds();
        **velocity += velocity_inc;
    }
}
pub fn get_gravitated(
    mut bodies: Query<(&mut Velocity,), With<Gravitation>>,
    gravity: Res<Gravity>,
    motion_scale: Res<MotionScale>,
    time: Res<Time>,
) {
    for (mut velocity,) in &mut bodies {
        let velocity_inc = **gravity * **motion_scale * time.delta_seconds();
        velocity.y += velocity_inc;
    }
}
pub fn scale_motion(
    mut motion_scale: ResMut<MotionScale>,
    time: Res<Time>,
    simul_start_time: Res<SimulStartTime>,
) {
    let time_since_simul_start: time::Duration =
        time.elapsed() - time::Duration::from(*simul_start_time);
    **motion_scale = time_since_simul_start.as_secs_f32().powf(0.25);
}
