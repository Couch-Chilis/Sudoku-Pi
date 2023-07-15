use bevy::prelude::*;

// Base colors.
pub const COLOR_CREAM: Color = Color::rgb(1., 254. / 255., 248. / 255.);
pub const COLOR_MAIN: Color = Color::rgb(169. / 255., 154. / 255., 55. / 255.);
pub const COLOR_MAIN_DARKEST: Color = Color::rgb(92. / 255., 84. / 255., 30. / 255.);
//pub const COLOR_MAIN_LIGHT: Color = Color::rgb(185. / 255., 168. / 255., 60. / 255.);
pub const COLOR_MAIN_LIGHT_04: Color = Color::rgba(185. / 255., 168. / 255., 60. / 255., 0.4);
pub const COLOR_MAIN_LIGHT_064: Color = Color::rgba(185. / 255., 168. / 255., 60. / 255., 0.64);
pub const COLOR_MAIN_POP_DARK: Color = Color::rgb(213. / 255., 11. / 255., 72. / 255.);
pub const COLOR_ORANGE: Color = Color::rgb(236. / 255., 117. / 255., 5. / 255.);

// Button colors.
pub const COLOR_BUTTON_TEXT: Color = Color::WHITE;
pub const COLOR_BUTTON_TEXT_PRESS: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_BUTTON_BACKGROUND: Color = COLOR_MAIN;
pub const COLOR_BUTTON_BACKGROUND_SELECTED: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_BUTTON_BACKGROUND_PRESS: Color = Color::WHITE;

pub const COLOR_SECONDARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_TEXT_SELECTED: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_TEXT_PRESS: Color = COLOR_CREAM;
pub const COLOR_SECONDARY_BUTTON_BACKGROUND: Color = COLOR_CREAM;
pub const COLOR_SECONDARY_BUTTON_BACKGROUND_PRESS: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_BORDER_SELECTED: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_BORDER_PRESS: Color = COLOR_CREAM;

pub const COLOR_TERNARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_TEXT_SELECTED: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_TERNARY_BUTTON_TEXT_PRESS: Color = Color::WHITE;
pub const COLOR_TERNARY_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const COLOR_TERNARY_BUTTON_BACKGROUND_PRESS: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_TERNARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_BORDER_SELECTED: Color = COLOR_MAIN_POP_DARK;
pub const COLOR_TERNARY_BUTTON_BORDER_PRESS: Color = Color::WHITE;

// Timer colors.
pub const COLOR_TIMER_BORDER: Color = COLOR_MAIN;
pub const COLOR_TIMER_TEXT: Color = COLOR_MAIN_DARKEST;

// Score color.
pub const COLOR_SCORE_TEXT: Color = COLOR_MAIN_POP_DARK;

// Wheel colors.
pub const COLOR_WHEEL_TOP_TEXT: Color = Color::WHITE;

// Toggle colors.
pub const COLOR_TOGGLE_OFF: Color = Color::WHITE;
pub const COLOR_TOGGLE_ON: Color = COLOR_MAIN_POP_DARK;

// Hint color.
pub const COLOR_HINT: Color = Color::rgb(163. / 255., 217. / 255., 1.);

// Board colors.
pub const COLOR_BOARD_LINE_THICK: Color = COLOR_MAIN_DARKEST;
pub const COLOR_BOARD_LINE_MEDIUM: Color = Color::rgb(185. / 255., 178. / 255., 129. / 255.);
pub const COLOR_BOARD_LINE_THIN: Color = Color::rgb(238. / 255., 235. / 255., 215. / 255.);

// Cell colors
pub const COLOR_CELL_SELECTION: Color = COLOR_MAIN;
pub const COLOR_CELL_SAME_NUMBER: Color = COLOR_MAIN_LIGHT_064;
pub const COLOR_CELL_HIGHLIGHT: Color = COLOR_MAIN_LIGHT_04;

// Cell size.
pub const CELL_SIZE: f32 = 0.111111;

// Highscores.
pub const MAX_NUM_HIGHSCORES: usize = 5;
