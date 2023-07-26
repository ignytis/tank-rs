use std::time::Duration;

use bevy::prelude::*;

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

    pub fn start_rotate(&mut self, dest: Quat, direction: bool, is_angle_increases: Option<bool>) {
        self.movement_mode = MovementMode::Rotate(dest, direction, is_angle_increases);
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