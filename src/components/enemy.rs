use bevy::prelude::*;

pub enum MovementMode {
    Move,
    Rotate(Quat), // arguments: desination angle
}

#[derive(Component)]
pub struct Enemy {
    pub movement_mode: MovementMode,
}

impl Enemy {
    pub fn start_move(&mut self) {
        self.movement_mode = MovementMode::Move;
    }

    pub fn start_rotate(&mut self, dest: Quat) {
        self.movement_mode = MovementMode::Rotate(dest);
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            movement_mode: MovementMode::Move,
        }
    }
}