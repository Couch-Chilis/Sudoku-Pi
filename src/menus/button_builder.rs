use crate::Fonts;
use crate::{constants::*, ui::*, utils::*};
use bevy::prelude::*;

pub struct ButtonBuilder {
    button_style: FlexItemStyle,
    text_style: TextStyle,
    text_transform: Transform,
    secondary_text_style: TextStyle,
    ternary_text_style: TextStyle,
    alternative_background_style: FlexItemStyle,
}

impl ButtonBuilder {
    pub fn new(fonts: &Fonts) -> Self {
        // Shared button styling.
        let button_style = FlexItemStyle::fixed_size(Val::Vmin(60.), Val::Vmin(14.))
            .with_margin(Size::all(Val::Vmin(2.)));

        // Shared text transform.
        let text_transform = Transform {
            scale: Vec3::new(0.002, 0.01, 1.),
            translation: Vec3::new(0., -0.08, 3.),
            ..default()
        };

        // Text styling for primary buttons.
        let text_style = TextStyle {
            font: fonts.medium.clone(),
            font_size: 60.,
            color: COLOR_BUTTON_TEXT,
        };

        // Text styling for secondary buttons.
        let secondary_text_style = TextStyle {
            font: fonts.medium.clone(),
            font_size: text_style.font_size,
            color: COLOR_SECONDARY_BUTTON_TEXT,
        };

        // Text styling for ternary buttons.
        let ternary_text_style = TextStyle {
            color: COLOR_TERNARY_BUTTON_TEXT,
            ..secondary_text_style.clone()
        };

        // Background style for secondary and ternary buttons.
        let alternative_background_style =
            FlexItemStyle::available_size().without_occupying_space();

        Self {
            button_style,
            text_style,
            text_transform,
            secondary_text_style,
            ternary_text_style,
            alternative_background_style,
        }
    }

    pub fn add_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((ButtonBundle::from_style(self.button_style.clone()), action))
            .with_children(|button| {
                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.text_style.clone()),
                    transform: self.text_transform.clone(),
                    ..default()
                });
            });
    }

    pub fn add_secondary_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                FlexBundle {
                    container: FlexContainerBundle {
                        style: FlexContainerStyle {
                            direction: FlexDirection::Row,
                            padding: Size::all(Val::Vmin(4.)),
                            ..default()
                        },
                        background: Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER),
                        transform: Transform::default_2d(),
                        ..default()
                    },
                    item: FlexItemBundle::from_style(self.button_style.clone()),
                },
                Button,
                ButtonType::Secondary,
                Interaction::default(),
                action,
            ))
            .with_children(|button| {
                button.spawn((
                    FlexItemBundle::from_style(self.alternative_background_style.clone()),
                    SpriteBundle {
                        sprite: Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
                        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                        ..default()
                    },
                ));

                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.secondary_text_style.clone()),
                    transform: self.text_transform.clone(),
                    ..default()
                });
            });
    }

    pub fn add_ternary_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                FlexBundle {
                    container: FlexContainerBundle {
                        style: FlexContainerStyle {
                            direction: FlexDirection::Row,
                            padding: Size::all(Val::Vmin(4.)),
                            ..default()
                        },
                        background: Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER),
                        transform: Transform::default_2d(),
                        ..default()
                    },
                    item: FlexItemBundle::from_style(self.button_style.clone()),
                },
                Button,
                ButtonType::Ternary,
                Interaction::default(),
                action,
            ))
            .with_children(|button| {
                button.spawn((
                    FlexItemBundle::from_style(self.alternative_background_style.clone()),
                    SpriteBundle {
                        sprite: Sprite::from_color(COLOR_TERNARY_BUTTON_BACKGROUND),
                        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                        ..default()
                    },
                ));

                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.ternary_text_style.clone()),
                    transform: self.text_transform.clone(),
                    ..default()
                });
            });
    }
}
