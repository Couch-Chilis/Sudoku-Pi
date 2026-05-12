use bevy::prelude::*;

// Base colors.
const COLOR_BAMBOO_SHOOT: Color = Color::srgb(204. / 255., 188. / 255., 42. / 255.);
const COLOR_CREAM: Color = Color::srgb(1., 254. / 255., 248. / 255.);
const COLOR_DEEP_BROWN: Color = Color::srgb(36. / 255., 21. / 255., 0.);
const COLOR_EGGSHELL: Color = Color::srgb(251. / 255., 249. / 255., 230. / 255.);
const COLOR_MAIN: Color = Color::srgb(169. / 255., 154. / 255., 55. / 255.);
const COLOR_MAIN_DARKER: Color = Color::srgb(139. / 255., 126. / 255., 45. / 255.);
const COLOR_MAIN_DARKEST: Color = Color::srgb(92. / 255., 84. / 255., 30. / 255.);
//const COLOR_MAIN_LIGHT: Color = Color::srgb(185. / 255., 168. / 255., 60. / 255.);
const COLOR_MUSTARD: Color = Color::srgb(229. / 255., 216. / 255., 97. / 255.);
const COLOR_ORANGE: Color = Color::srgb(236. / 255., 117. / 255., 5. / 255.);
const COLOR_POP_DARK: Color = Color::srgb(213. / 255., 11. / 255., 72. / 255.);
const COLOR_POP_FOCUS: Color = Color::srgb(245. / 255., 55. / 255., 112. / 255.);

#[derive(Clone, Resource)]
pub struct Theme {
    // Background color.
    pub background: Color,

    // Regular text.
    pub text: Color,
    pub text_highlight: Color,

    // Button colors.
    pub button_text: Color,
    pub button_text_press: Color,
    pub button_background: Color,
    pub button_background_selected: Color,
    pub button_background_press: Color,

    pub secondary_button_text: Color,
    pub secondary_button_text_selected: Color,
    pub secondary_button_text_press: Color,
    pub secondary_button_background: Color,
    pub secondary_button_background_press: Color,
    pub secondary_button_border: Color,
    pub secondary_button_border_selected: Color,
    pub secondary_button_border_press: Color,

    pub ternary_button_text: Color,
    pub ternary_button_text_selected: Color,
    pub ternary_button_text_press: Color,
    pub ternary_button_background: Color,
    pub ternary_button_background_press: Color,
    pub ternary_button_border: Color,
    pub ternary_button_border_selected: Color,
    pub ternary_button_border_press: Color,

    // Timer colors.
    pub timer_border: Color,
    pub timer_text: Color,

    // Score color.
    pub score_text: Color,

    // Wheel colors.
    pub wheel_top_text: Color,

    // Hint color.
    pub hint: Color,

    // Mistake color.
    pub mistake: Color,

    // Board colors.
    pub board_number: Color,
    pub board_prefill_number: Color,
    pub board_line_thick: Color,
    pub board_line_medium: Color,
    pub board_line_thin: Color,

    // Cell colors
    pub cell_selection: Color,
    pub cell_same_number: Color,
    pub cell_highlight: Color,
}

impl Theme {
    pub fn background(&self) -> Color {
        self.background
    }

    pub fn board_line_medium(&self) -> Color {
        self.board_line_medium
    }

    pub fn board_line_thin(&self) -> Color {
        self.board_line_thin
    }

    pub fn score_text(&self) -> Color {
        self.score_text
    }

    pub fn text(&self) -> Color {
        self.text
    }

    pub fn text_highlight(&self) -> Color {
        self.text_highlight
    }

    pub fn timer_border(&self) -> Color {
        self.timer_border
    }

    pub fn timer_text(&self) -> Color {
        self.timer_text
    }
}

pub const LIGHT_THEME: Theme = Theme {
    background: Color::WHITE,

    text: COLOR_MAIN_DARKER,
    text_highlight: COLOR_POP_FOCUS,

    button_text: Color::WHITE,
    button_text_press: COLOR_POP_DARK,
    button_background: COLOR_MAIN,
    button_background_selected: COLOR_POP_DARK,
    button_background_press: Color::WHITE,

    secondary_button_text: COLOR_MAIN,
    secondary_button_text_selected: COLOR_POP_DARK,
    secondary_button_text_press: COLOR_CREAM,
    secondary_button_background: COLOR_CREAM,
    secondary_button_background_press: COLOR_POP_DARK,
    secondary_button_border: COLOR_MAIN,
    secondary_button_border_selected: COLOR_POP_DARK,
    secondary_button_border_press: COLOR_CREAM,

    ternary_button_text: COLOR_MAIN,
    ternary_button_text_selected: COLOR_POP_DARK,
    ternary_button_text_press: Color::WHITE,
    ternary_button_background: Color::WHITE,
    ternary_button_background_press: COLOR_POP_DARK,
    ternary_button_border: COLOR_MAIN,
    ternary_button_border_selected: COLOR_POP_DARK,
    ternary_button_border_press: Color::WHITE,

    timer_border: COLOR_MAIN,
    timer_text: COLOR_MAIN_DARKEST,

    score_text: COLOR_POP_DARK,

    wheel_top_text: Color::WHITE,

    hint: Color::srgb(163. / 255., 217. / 255., 1.),

    mistake: COLOR_POP_DARK,

    board_number: Color::BLACK,
    board_prefill_number: Color::linear_rgb(0., 0., 1.),
    board_line_thick: COLOR_MAIN_DARKEST,
    board_line_medium: Color::srgb(185. / 255., 178. / 255., 129. / 255.),
    board_line_thin: Color::srgb(238. / 255., 235. / 255., 215. / 255.),

    cell_selection: COLOR_BAMBOO_SHOOT,
    cell_same_number: COLOR_MUSTARD,
    cell_highlight: COLOR_EGGSHELL,
};

pub const DARK_THEME: Theme = Theme {
    background: COLOR_DEEP_BROWN,

    text: COLOR_MAIN_DARKER,
    text_highlight: COLOR_ORANGE,

    button_text: Color::WHITE,
    button_text_press: COLOR_POP_DARK,
    button_background: COLOR_MAIN,
    button_background_selected: COLOR_ORANGE,
    button_background_press: Color::WHITE,

    secondary_button_text: COLOR_MAIN,
    secondary_button_text_selected: COLOR_POP_DARK,
    secondary_button_text_press: COLOR_CREAM,
    secondary_button_background: COLOR_DEEP_BROWN,
    secondary_button_background_press: COLOR_ORANGE,
    secondary_button_border: COLOR_MAIN,
    secondary_button_border_selected: COLOR_POP_DARK,
    secondary_button_border_press: COLOR_CREAM,

    ternary_button_text: COLOR_MAIN,
    ternary_button_text_selected: COLOR_POP_DARK,
    ternary_button_text_press: Color::WHITE,
    ternary_button_background: COLOR_DEEP_BROWN,
    ternary_button_background_press: COLOR_ORANGE,
    ternary_button_border: COLOR_MAIN,
    ternary_button_border_selected: COLOR_POP_DARK,
    ternary_button_border_press: Color::WHITE,

    timer_border: COLOR_MAIN_DARKEST,
    timer_text: COLOR_MAIN,

    score_text: COLOR_POP_DARK,

    wheel_top_text: Color::WHITE,

    hint: Color::srgb(163. / 255., 217. / 255., 1.),

    mistake: COLOR_POP_FOCUS,

    board_number: Color::WHITE,
    board_prefill_number: Color::linear_rgb(0., 0., 1.),
    board_line_thick: COLOR_MAIN_DARKEST,
    board_line_medium: Color::srgb(185. / 255., 178. / 255., 129. / 255.),
    board_line_thin: Color::srgb(238. / 255., 235. / 255., 215. / 255.),

    cell_selection: COLOR_BAMBOO_SHOOT,
    cell_same_number: COLOR_MUSTARD,
    cell_highlight: COLOR_EGGSHELL,
};
