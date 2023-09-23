use bevy::prelude::*;

/// An unbreakable wall
#[derive(Component)]
pub struct WallUnbreakable {}

impl Default for WallUnbreakable {
    fn default() -> Self {
        WallUnbreakable {}
    }
}