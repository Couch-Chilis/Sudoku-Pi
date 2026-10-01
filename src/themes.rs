use bevy::prelude::*;
use bevy::window::WindowTheme;

// Base colors.
const COLOR_BAMBOO_SHOOT: Color = Color::srgb(204. / 255., 188. / 255., 42. / 255.);
const COLOR_CREAM: Color = Color::srgb(1., 254. / 255., 248. / 255.);
const COLOR_DEEP_BROWN: Color = Color::srgb(38. / 255., 30. / 255., 27. / 255.);
const COLOR_LIGHTER_BROWN: Color = Color::srgb(60. / 255., 49. / 255., 44. / 255.);
const COLOR_EGGSHELL: Color = Color::srgb(251. / 255., 249. / 255., 230. / 255.);
const COLOR_MAIN: Color = Color::srgb(169. / 255., 154. / 255., 55. / 255.);
const COLOR_MAIN_DARKER: Color = Color::srgb(139. / 255., 126. / 255., 45. / 255.);
const COLOR_MAIN_DARKEST: Color = Color::srgb(92. / 255., 84. / 255., 30. / 255.);
//const COLOR_MAIN_LIGHT: Color = Color::srgb(185. / 255., 168. / 255., 60. / 255.);
const COLOR_MUSTARD: Color = Color::srgb(229. / 255., 216. / 255., 97. / 255.);
const COLOR_POP_DARK: Color = Color::srgb(213. / 255., 11. / 255., 72. / 255.);
const COLOR_POP_FOCUS: Color = Color::srgb(245. / 255., 55. / 255., 112. / 255.);

/// Tracks the host (operating system) theme so the app can follow it while the
/// theme setting is [`crate::settings::ThemeSetting::System`].
#[derive(Default, Resource)]
pub struct HostTheme(pub Option<WindowTheme>);

#[derive(Clone, Resource)]
pub struct Theme {
    // Background color.
    pub background: Color,

    // Regular text.
    pub text: Color,
    pub text_highlight: Color,

    // Scroll text (highscores quote and author).
    pub scroll_text: Color,

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

    pub fn is_dark_mode(&self) -> bool {
        self.background == COLOR_DEEP_BROWN
    }

    pub fn mix(&self, other: &Self, factor: f32) -> Self {
        Self {
            background: self.background.mix(&other.background, factor),
            text: self.text.mix(&other.text, factor),
            text_highlight: self.text_highlight.mix(&other.text_highlight, factor),
            scroll_text: self.scroll_text.mix(&other.scroll_text, factor),
            button_text: self.button_text.mix(&other.button_text, factor),
            button_text_press: self.button_text_press.mix(&other.button_text_press, factor),
            button_background: self.button_background.mix(&other.button_background, factor),
            button_background_selected: self
                .button_background_selected
                .mix(&other.button_background_selected, factor),
            button_background_press: self
                .button_background_press
                .mix(&other.button_background_press, factor),
            secondary_button_text: self
                .secondary_button_text
                .mix(&other.secondary_button_text, factor),
            secondary_button_text_selected: self
                .secondary_button_text_selected
                .mix(&other.secondary_button_text_selected, factor),
            secondary_button_text_press: self
                .secondary_button_text_press
                .mix(&other.secondary_button_text_press, factor),
            secondary_button_background: self
                .secondary_button_background
                .mix(&other.secondary_button_background, factor),
            secondary_button_background_press: self
                .secondary_button_background_press
                .mix(&other.secondary_button_background_press, factor),
            secondary_button_border: self
                .secondary_button_border
                .mix(&other.secondary_button_border, factor),
            secondary_button_border_selected: self
                .secondary_button_border_selected
                .mix(&other.secondary_button_border_selected, factor),
            secondary_button_border_press: self
                .secondary_button_border_press
                .mix(&other.secondary_button_border_press, factor),
            ternary_button_text: self
                .ternary_button_text
                .mix(&other.ternary_button_text, factor),
            ternary_button_text_selected: self
                .ternary_button_text_selected
                .mix(&other.ternary_button_text_selected, factor),
            ternary_button_text_press: self
                .ternary_button_text_press
                .mix(&other.ternary_button_text_press, factor),
            ternary_button_background: self
                .ternary_button_background
                .mix(&other.ternary_button_background, factor),
            ternary_button_background_press: self
                .ternary_button_background_press
                .mix(&other.ternary_button_background_press, factor),
            ternary_button_border: self
                .ternary_button_border
                .mix(&other.ternary_button_border, factor),
            ternary_button_border_selected: self
                .ternary_button_border_selected
                .mix(&other.ternary_button_border_selected, factor),
            ternary_button_border_press: self
                .ternary_button_border_press
                .mix(&other.ternary_button_border_press, factor),
            timer_border: self.timer_border.mix(&other.timer_border, factor),
            timer_text: self.timer_text.mix(&other.timer_text, factor),
            score_text: self.score_text.mix(&other.score_text, factor),
            wheel_top_text: self.wheel_top_text.mix(&other.wheel_top_text, factor),
            hint: self.hint.mix(&other.hint, factor),
            mistake: self.mistake.mix(&other.mistake, factor),
            board_number: self.board_number.mix(&other.board_number, factor),
            board_prefill_number: self
                .board_prefill_number
                .mix(&other.board_prefill_number, factor),
            board_line_thick: self.board_line_thick.mix(&other.board_line_thick, factor),
            board_line_medium: self.board_line_medium.mix(&other.board_line_medium, factor),
            board_line_thin: self.board_line_thin.mix(&other.board_line_thin, factor),
            cell_selection: self.cell_selection.mix(&other.cell_selection, factor),
            cell_same_number: self.cell_same_number.mix(&other.cell_same_number, factor),
            cell_highlight: self.cell_highlight.mix(&other.cell_highlight, factor),
        }
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
    scroll_text: Color::BLACK,

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

    text: COLOR_MAIN,
    text_highlight: COLOR_POP_FOCUS,
    scroll_text: COLOR_EGGSHELL,

    button_text: Color::WHITE,
    button_text_press: Color::WHITE,
    button_background: COLOR_MAIN,
    button_background_selected: COLOR_POP_FOCUS,
    button_background_press: COLOR_POP_DARK,

    secondary_button_text: COLOR_MAIN,
    secondary_button_text_selected: COLOR_POP_FOCUS,
    secondary_button_text_press: Color::WHITE,
    secondary_button_background: COLOR_LIGHTER_BROWN,
    secondary_button_background_press: COLOR_POP_DARK,
    secondary_button_border: COLOR_MAIN,
    secondary_button_border_selected: COLOR_POP_FOCUS,
    secondary_button_border_press: COLOR_POP_DARK,

    ternary_button_text: COLOR_MAIN,
    ternary_button_text_selected: COLOR_POP_FOCUS,
    ternary_button_text_press: Color::WHITE,
    ternary_button_background: COLOR_DEEP_BROWN,
    ternary_button_background_press: COLOR_POP_DARK,
    ternary_button_border: COLOR_MAIN,
    ternary_button_border_selected: COLOR_POP_FOCUS,
    ternary_button_border_press: COLOR_POP_DARK,

    timer_border: COLOR_MAIN_DARKEST,
    timer_text: COLOR_MAIN,

    score_text: COLOR_POP_FOCUS,

    wheel_top_text: Color::WHITE,

    hint: Color::srgb(163. / 255., 217. / 255., 1.),

    mistake: COLOR_POP_FOCUS,

    board_number: COLOR_EGGSHELL,
    board_prefill_number: Color::linear_rgb(0., 0., 1.),
    board_line_thick: COLOR_MAIN,
    board_line_medium: COLOR_MAIN_DARKER,
    board_line_thin: COLOR_MAIN_DARKEST,

    cell_selection: COLOR_BAMBOO_SHOOT,
    cell_same_number: COLOR_MAIN_DARKEST,
    cell_highlight: COLOR_LIGHTER_BROWN,
};
