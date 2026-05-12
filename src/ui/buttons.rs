use bevy::prelude::*;

use crate::themes::Theme;

use super::{flex::*, Interaction};

pub const BORDER_THICKNESS: Val = Val::Pixel(1);

/// Marker for buttons.
#[derive(Clone, Component, Default)]
pub struct Button;

/// Marker for button backgrounds (only used for buttons that have a border).
#[derive(Clone, Component, Default)]
pub struct ButtonBackground;

/// Marks a button as secondary.
#[derive(Clone, Component, Default)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Ternary,
}

/// A UI button with text that is also a flex item.
#[derive(Bundle, Clone, Default)]
pub struct ButtonBundle {
    pub button: Button,
    pub flex: FlexBundle,
    pub interaction: Interaction,
}

impl ButtonBundle {
    pub fn from_theme_and_style(theme: &Theme, style: FlexItemStyle) -> Self {
        Self {
            flex: FlexBundle::new(style, FlexContainerStyle::row())
                .with_background_color(theme.button_background),
            ..default()
        }
    }
}
