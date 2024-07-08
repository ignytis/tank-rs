use bevy::color::Color;

pub const FRAMERATE_MAX: f64 = 90.;

pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const WINDOW_X_MIN: f32 = WINDOW_WIDTH / -2.0;
pub const WINDOW_X_MAX: f32 = WINDOW_WIDTH / 2.0;
pub const WINDOW_Y_MIN: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_Y_MAX: f32 = WINDOW_HEIGHT / 2.0;

pub const HUD_WIDTH: f32 = 200.0;
pub const WINDOW_MAIN_AREA_RIGHT_X: f32 = WINDOW_X_MAX  - HUD_WIDTH;

/// Width and height of a tank in pixels. Should equal to sprite dimensions
pub const TANK_DIMENSION: u32 = 50;

// Sprites with higher z-index will be on top of ones with lower z-index in case of overlap
pub const Z_INDEX_GROUND: f32 = 0.0;
pub const Z_INDEX_SPAWN: f32 = 10.0;
pub const Z_INDEX_SHELL: f32 = 40.0;
pub const Z_INDEX_TANK: f32 = 50.0;
pub const Z_INDEX_TANK_EXPLOSION: f32 = 70.0;

pub const MAX_ENEMIES: usize = 5;

pub const LIVES_ENEMY: u8 = 10;
pub const LIVES_PLAYER: u8 = 5;

pub const COLOR_RED: Color = Color::linear_rgb(1.0, 0.0, 0.0);