use bevy::prelude::*;

pub const BUTTON_TEXT: Color = Color::WHITE;
pub const NORMAL_BUTTON: Color = Color::rgb(0.733, 0.667, 0.6);
pub const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.733, 0.667);
pub const PRESSED_BUTTON: Color = Color::rgb(0.867, 0.8, 0.733);

pub const SECONDARY_BUTTON: Color = Color::NONE;
pub const SECONDARY_BUTTON_TEXT: Color = Color::rgb(0.067, 0.0, 0.0);
pub const SECONDARY_HOVERED_BUTTON: Color = Color::rgba(0.8, 0.733, 0.667, 0.3);
pub const SECONDARY_PRESSED_BUTTON_TEXT: Color = Color::rgb(0.4, 0.333, 0.333);

pub const CELL_SIZE: f32 = 0.111111;
pub const CELL_SCALE: Vec3 = Vec3::new(CELL_SIZE, CELL_SIZE, 1.);
