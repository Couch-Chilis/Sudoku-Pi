use super::{flex::*, InitialSelection, Interaction};
use crate::{constants::*, ResourceBag};
use bevy::prelude::*;

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

/// Marker for button borders.
#[derive(Clone, Component)]
pub struct Border;

/// A UI button with text that is also a flex item.
#[derive(Bundle, Clone, Default)]
pub struct ButtonBundle {
    pub button: Button,
    pub flex: FlexBundle,
    pub interaction: Interaction,
}

impl ButtonBundle {
    pub fn from_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexBundle::new(style, FlexContainerStyle::row())
                .with_background_color(COLOR_BUTTON_BACKGROUND),
            ..default()
        }
    }
}

pub struct ButtonBuilder {
    button_style: FlexItemStyle,
    text_style: TextStyle,
    secondary_text_style: TextStyle,
    alternative_background_style: FlexItemStyle,
}

impl ButtonBuilder {
    pub fn new(resources: &ResourceBag, button_style: FlexItemStyle, font_size: f32) -> Self {
        // Text styling for primary buttons.
        let text_style = TextStyle {
            font: resources.fonts.medium.clone(),
            font_size,
            color: COLOR_BUTTON_TEXT,
        };

        // Text styling for secondary buttons.
        let secondary_text_style = TextStyle {
            font: resources.fonts.medium.clone(),
            font_size,
            color: COLOR_SECONDARY_BUTTON_TEXT,
        };

        // Background style for secondary and ternary buttons.
        let alternative_background_style =
            FlexItemStyle::available_size().without_occupying_space();

        Self {
            button_style,
            text_style,
            secondary_text_style,
            alternative_background_style,
        }
    }

    pub fn build_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((ButtonBundle::from_style(self.button_style.clone()), action))
            .with_children(|button| {
                button.spawn(FlexTextBundle::from_text(Text::from_section(
                    text,
                    self.text_style.clone(),
                )));
            });
    }

    pub fn build_selected_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                InitialSelection,
                ButtonBundle {
                    flex: FlexBundle::from_item_style(self.button_style.clone()),
                    interaction: Interaction::Selected,
                    ..default()
                },
                action,
            ))
            .with_children(|button| {
                button.spawn(FlexTextBundle::from_text(Text::from_section(
                    text,
                    self.text_style.clone(),
                )));
            });
    }

    pub fn build_secondary_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                ButtonBundle {
                    flex: FlexBundle::new(
                        self.button_style.clone(),
                        FlexContainerStyle::row().with_padding(Sides::all(BORDER_THICKNESS)),
                    )
                    .with_background_color(COLOR_SECONDARY_BUTTON_BORDER),
                    ..default()
                },
                ButtonType::Secondary,
                action,
            ))
            .with_children(|border| {
                border
                    .spawn((
                        ButtonBackground,
                        FlexBundle::new(
                            self.alternative_background_style.clone(),
                            FlexContainerStyle::row(),
                        )
                        .with_background_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
                    ))
                    .with_children(|background| {
                        background.spawn(FlexTextBundle::from_text(Text::from_section(
                            text,
                            self.secondary_text_style.clone(),
                        )));
                    });
            });
    }
}
