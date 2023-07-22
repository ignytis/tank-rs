use std::f32::consts::PI;

use bevy::prelude::*;

/// Azimuth = positive float from 0.0 to 2*PI. Increments clockwise
pub fn quat_to_azimuth(rotation: Quat) -> f32 {
    let (axis, angle) = rotation.to_axis_angle();
    if axis.z >= 0. {
        2. * PI - angle
    } else {
        angle
        
    }
}