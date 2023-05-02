use super::{Button, ButtonType, ComputedPosition};
use crate::{constants::*, utils::SpriteExt};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Hovered,
    JustPressed,
    Pressed,
}

pub fn mouse_interaction(
    mut interaction_query: Query<(&mut Interaction, &ComputedPosition)>,
    mouse_buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_position) = window_query.get_single().ok().and_then(|window| window.cursor_position()) else {
        return;
    };

    for (mut interaction, computed_position) in &mut interaction_query {
        let new_interaction = if computed_position.contains(cursor_position) {
            if mouse_buttons.just_pressed(MouseButton::Left) {
                Interaction::JustPressed
            } else if mouse_buttons.pressed(MouseButton::Left) {
                Interaction::Pressed
            } else {
                Interaction::Hovered
            }
        } else {
            Interaction::None
        };

        if *interaction != new_interaction {
            *interaction = new_interaction;
        }
    }
}

// Handles changing button color based on mouse interaction.
pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut Sprite, Option<&ButtonType>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, mut sprite, button_type) in &mut interaction_query {
        match button_type.cloned().unwrap_or_default() {
            ButtonType::Primary => {
                *sprite = match *interaction {
                    Interaction::JustPressed | Interaction::Pressed => {
                        Sprite::from_color(COLOR_BUTTON_BACKGROUND_PRESS)
                    }
                    Interaction::Hovered => Sprite::from_color(COLOR_BUTTON_BACKGROUND_HOVER),
                    Interaction::None => Sprite::from_color(COLOR_BUTTON_BACKGROUND),
                };
            }
            ButtonType::Secondary => {
                *sprite = match *interaction {
                    Interaction::JustPressed | Interaction::Pressed => {
                        Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER_PRESS)
                    }
                    Interaction::Hovered => Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER_HOVER),
                    Interaction::None => Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER),
                };

                if let Some(mut text) = children
                    .get(1)
                    .and_then(|child| text_query.get_mut(*child).ok())
                {
                    text.sections[0].style.color = match *interaction {
                        Interaction::JustPressed | Interaction::Pressed => {
                            COLOR_SECONDARY_BUTTON_TEXT_PRESS.into()
                        }
                        Interaction::Hovered => COLOR_SECONDARY_BUTTON_TEXT_HOVER.into(),
                        Interaction::None => COLOR_SECONDARY_BUTTON_TEXT.into(),
                    };
                }
            }
            ButtonType::Ternary => {
                *sprite = match *interaction {
                    Interaction::JustPressed | Interaction::Pressed => {
                        Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER_PRESS)
                    }
                    Interaction::Hovered => Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER_HOVER),
                    Interaction::None => Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER),
                };

                if let Some(mut text) = children
                    .get(1)
                    .and_then(|child| text_query.get_mut(*child).ok())
                {
                    text.sections[0].style.color = match *interaction {
                        Interaction::JustPressed | Interaction::Pressed => {
                            COLOR_TERNARY_BUTTON_TEXT_PRESS.into()
                        }
                        Interaction::Hovered => COLOR_TERNARY_BUTTON_TEXT_HOVER.into(),
                        Interaction::None => COLOR_TERNARY_BUTTON_TEXT.into(),
                    };
                }
            }
        }
    }
}
