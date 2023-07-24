use std::f32::consts::PI;

use bevy::prelude::*;

const FULL_CIRCLE_RAD: f32 = 2. * PI;

/// Azimuth = positive float from 0.0 to 2*PI. Increments clockwise
pub fn quat_to_azimuth(rotation: Quat) -> f32 {
    let (axis, angle) = rotation.to_axis_angle();
    let mut az = if axis.z >= 0. {
        FULL_CIRCLE_RAD - angle
    } else {
        angle
    };

    while az >= FULL_CIRCLE_RAD {
        az -= FULL_CIRCLE_RAD;
    }
    az
}

/// true = clockwise, false = counter clockwise
pub fn get_closer_direction(from: Quat, to: Quat) -> bool {
    let mut from_az = quat_to_azimuth(from);
    let to_az = quat_to_azimuth(to);

    if from_az < to_az {
        from_az += FULL_CIRCLE_RAD;
    }

    from_az - to_az >= PI
}

/// Transforms azimuth (0.0..2*PI radians, clockwise) to Quat with negative Z axis (from viewer to screen)
pub fn azimuth_to_quat_negative_z(azimuth: f32) -> Quat {
    Quat::from_axis_angle(Vec3::NEG_Z, azimuth)
}