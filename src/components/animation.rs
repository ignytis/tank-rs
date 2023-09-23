use bevy::prelude::*;

// from https://bevyengine.org/examples/2D%20Rendering/sprite-sheet/

#[derive(Component)]
pub struct AnimationData {
    pub first: usize,
    pub last: usize,
    pub delete_after_last_frame: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);