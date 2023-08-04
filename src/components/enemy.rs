use std::time::Duration;

use bevy::{prelude::*, ecs::query::ReadOnlyWorldQuery};

use crate::geometry::{azimuth_to_quat_negative_z, get_closer_direction};

pub enum MovementMode {
    Move,
    // arguments:
    // - desination angle,
    // - direction (true = clockwise),
    // - true if angle increases, false otherwise
    Rotate(Quat, bool, Option<bool>),
}

#[derive(Component)]
pub struct Enemy {
    pub azimuth: f32,
    pub movement_mode: MovementMode,
    pub shoot_timer: Timer,
}

impl Enemy {
    pub fn start_move(&mut self) {
        self.movement_mode = MovementMode::Move;
    }

    pub fn start_rotate(&mut self, dest: Quat) {
        let self_quat =azimuth_to_quat_negative_z(self.azimuth);
        let direction = get_closer_direction(self_quat, dest);
        self.movement_mode = MovementMode::Rotate(dest, direction, None);
    }

    // TODO: mby instead of start_rotate and continue_rotate make
    // MovementMode::StartRotate and MovementMode::ContinueRotate enums?
    pub fn continue_rotate(&mut self, dest: Quat, is_angle_increases: bool) {
        let self_quat = azimuth_to_quat_negative_z(self.azimuth);
        let direction = get_closer_direction(self_quat, dest);
        self.movement_mode = MovementMode::Rotate(dest, direction, Some(is_angle_increases));
    }

    pub fn is_rotating(&self) -> bool {
        match self.movement_mode {
            MovementMode::Rotate(_, _, _) => true,
            _ => false,
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            azimuth: 0.,
            movement_mode: MovementMode::Move,
            shoot_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)
        }
    }
}