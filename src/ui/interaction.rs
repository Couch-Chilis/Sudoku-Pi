use super::{Button, ButtonBackground, ButtonType, ComputedPosition};
use crate::{constants::*, utils::SpriteExt, ScreenState};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Selected,
    Pressed,
}

pub fn mouse_interaction(
    mut interaction_query: Query<(
        Entity,
        &mut Interaction,
        &ComputedPosition,
        &ComputedVisibility,
    )>,
    mouse_buttons: Res<Input<MouseButton>>,
    screen: Res<State<ScreenState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_position) = window_query.get_single().ok().and_then(|window| window.cursor_position()) else {
        return;
    };

    let selected_entity = interaction_query
        .iter()
        .find(|(_, _, computed_position, computed_visibility)| {
            computed_position.screens.contains(&screen.0)
                && computed_visibility.is_visible()
                && computed_position.contains(cursor_position)
        })
        .map(|(entity, ..)| entity);

    for (entity, mut interaction, computed_position, _) in &mut interaction_query {
        let new_interaction = match selected_entity {
            Some(selected_entity) => {
                if selected_entity == entity {
                    if mouse_buttons.just_pressed(MouseButton::Left) {
                        Interaction::Pressed
                    } else {
                        Interaction::Selected
                    }
                } else if computed_position.screens.contains(&screen.0) {
                    Interaction::None
                } else {
                    interaction.clone()
                }
            }
            None => interaction.clone(),
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
        (
            Changed<Interaction>,
            With<Button>,
            Without<ButtonBackground>,
        ),
    >,
    mut background_query: Query<(&mut Sprite, &Children), With<ButtonBackground>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, mut sprite, button_type) in &mut interaction_query {
        match button_type.cloned().unwrap_or_default() {
            ButtonType::Primary => {
                *sprite = match *interaction {
                    Interaction::Pressed => Sprite::from_color(COLOR_BUTTON_BACKGROUND_PRESS),
                    Interaction::Selected => Sprite::from_color(COLOR_BUTTON_BACKGROUND_SELECTED),
                    Interaction::None => Sprite::from_color(COLOR_BUTTON_BACKGROUND),
                };

                if let Some(mut text) = children
                    .get(0)
                    .and_then(|child| text_query.get_mut(*child).ok())
                {
                    text.sections[0].style.color = match *interaction {
                        Interaction::Pressed => COLOR_BUTTON_TEXT_PRESS,
                        _ => COLOR_BUTTON_TEXT,
                    };
                }
            }
            ButtonType::Secondary => {
                *sprite = match *interaction {
                    Interaction::Pressed => Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER_PRESS),
                    Interaction::Selected => {
                        Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER_SELECTED)
                    }
                    Interaction::None => Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER),
                };

                if let Some((mut background, children)) = children
                    .get(0)
                    .and_then(|child| background_query.get_mut(*child).ok())
                {
                    *background = match *interaction {
                        Interaction::Pressed => {
                            Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND_PRESS)
                        }
                        _ => Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
                    };

                    if let Some(mut text) = children
                        .get(0)
                        .and_then(|child| text_query.get_mut(*child).ok())
                    {
                        text.sections[0].style.color = match *interaction {
                            Interaction::Pressed => COLOR_SECONDARY_BUTTON_TEXT_PRESS,
                            Interaction::Selected => COLOR_SECONDARY_BUTTON_TEXT_SELECTED,
                            Interaction::None => COLOR_SECONDARY_BUTTON_TEXT,
                        };
                    }
                }
            }
            ButtonType::Ternary => {
                *sprite = match *interaction {
                    Interaction::Pressed => Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER_PRESS),
                    Interaction::Selected => {
                        Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER_SELECTED)
                    }
                    Interaction::None => Sprite::from_color(COLOR_TERNARY_BUTTON_BORDER),
                };

                if let Some((mut background, children)) = children
                    .get(0)
                    .and_then(|child| background_query.get_mut(*child).ok())
                {
                    *background = match *interaction {
                        Interaction::Pressed => {
                            Sprite::from_color(COLOR_TERNARY_BUTTON_BACKGROUND_PRESS)
                        }
                        _ => Sprite::from_color(COLOR_TERNARY_BUTTON_BACKGROUND),
                    };

                    if let Some(mut text) = children
                        .get(0)
                        .and_then(|child| text_query.get_mut(*child).ok())
                    {
                        text.sections[0].style.color = match *interaction {
                            Interaction::Pressed => COLOR_TERNARY_BUTTON_TEXT_PRESS,
                            Interaction::Selected => COLOR_TERNARY_BUTTON_TEXT_SELECTED,
                            Interaction::None => COLOR_TERNARY_BUTTON_TEXT,
                        };
                    }
                }
            }
        }
    }
}
