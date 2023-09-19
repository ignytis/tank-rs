use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

/// Azimuth = positive float from 0.0 to 2*PI. Increments clockwise
pub fn quat_to_azimuth(rotation: Quat) -> f32 {
    let (axis, angle) = rotation.to_axis_angle();
    let mut az = if axis.z >= 0. {
        TAU - angle
    } else {
        angle
    };

    while az >= TAU {
        az -= TAU;
    }
    az
}

/// true = clockwise, false = counter clockwise
pub fn get_closer_direction(from: Quat, to: Quat) -> bool {
    let mut from_az = quat_to_azimuth(from);
    let to_az = quat_to_azimuth(to);

    if from_az < to_az {
        from_az += TAU;
    }

    from_az - to_az >= PI
}

/// Transforms azimuth (0.0..2*PI radians, clockwise) to Quat with negative Z axis (from viewer to screen)
pub fn azimuth_to_quat_negative_z(azimuth: f32) -> Quat {
    let mut a = azimuth;
    while a > TAU {
        a -= TAU;
    }
    Quat::from_axis_angle(Vec3::NEG_Z, a)
}

pub fn vec3_to_azimuth(v: Vec3) -> f32 {
    let mut angle = v.angle_between(Vec3::Y);
    while angle > TAU {
        angle -= TAU;
    }
    if v.x >= 0. && v.x <= PI {
        angle
    } else {
        angle + PI
    }
}