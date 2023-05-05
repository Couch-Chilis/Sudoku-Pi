use bevy::prelude::*;

// Base colors.
pub const COLOR_BACKGROUND: Color = Color::rgb(255. / 255., 254. / 255., 248. / 255.);
pub const COLOR_MAIN: Color = Color::rgb(169. / 255., 154. / 255., 55. / 255.);
pub const COLOR_MAIN_OPACITY_08: Color = Color::rgba(169. / 255., 154. / 255., 55. / 255., 0.8);
pub const COLOR_MAIN_DARKEST: Color = Color::rgb(92. / 255., 84. / 255., 30. / 255.);
pub const COLOR_MAIN_POP_DARK: Color = Color::rgb(213. / 255., 11. / 255., 72. / 255.);

// Button colors.
pub const COLOR_BUTTON_TEXT: Color = Color::WHITE;
pub const COLOR_BUTTON_BACKGROUND: Color = COLOR_MAIN;
pub const COLOR_BUTTON_BACKGROUND_HOVER: Color = COLOR_MAIN_OPACITY_08;
pub const COLOR_BUTTON_BACKGROUND_PRESS: Color = COLOR_MAIN_POP_DARK;

pub const COLOR_SECONDARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_TEXT_HOVER: Color = COLOR_MAIN_OPACITY_08;
pub const COLOR_SECONDARY_BUTTON_TEXT_PRESS: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_BACKGROUND: Color = COLOR_BACKGROUND;
pub const COLOR_SECONDARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_BORDER_HOVER: Color = COLOR_MAIN_OPACITY_08;
pub const COLOR_SECONDARY_BUTTON_BORDER_PRESS: Color = COLOR_MAIN_POP_DARK;

pub const COLOR_TERNARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_TEXT_HOVER: Color = COLOR_MAIN_OPACITY_08;
pub const COLOR_TERNARY_BUTTON_TEXT_PRESS: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_TERNARY_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const COLOR_TERNARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_BORDER_HOVER: Color = COLOR_MAIN_OPACITY_08;
pub const COLOR_TERNARY_BUTTON_BORDER_PRESS: Color = COLOR_MAIN_POP_DARK;

// Timer colors.
pub const COLOR_TIMER_BORDER: Color = COLOR_MAIN;
pub const COLOR_TIMER_TEXT: Color = COLOR_MAIN_DARKEST;

// Score color.
pub const COLOR_SCORE_TEXT: Color = COLOR_MAIN_POP_DARK;

// Toggle colors.
pub const COLOR_TOGGLE_OFF: Color = Color::WHITE;
pub const COLOR_TOGGLE_ON: Color = COLOR_MAIN_POP_DARK;

/// Board colors.
pub const COLOR_BOARD_LINE_THICK: Color = COLOR_MAIN_DARKEST;
pub const COLOR_BOARD_LINE_MEDIUM: Color = Color::rgb(185. / 255., 178. / 255., 129. / 255.);
pub const COLOR_BOARD_LINE_THIN: Color = Color::rgb(238. / 255., 235. / 255., 215. / 255.);

// Cells.
pub const CELL_SIZE: f32 = 0.111111;
pub const CELL_SCALE: Vec3 = Vec3::new(CELL_SIZE, CELL_SIZE, 1.);
