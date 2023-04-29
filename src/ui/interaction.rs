use super::{Button, ComputedPosition, Secondary};
use crate::{constants::*, utils::SpriteExt};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Component, Default, Eq, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Hovered,
    Clicked,
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
            if mouse_buttons.pressed(MouseButton::Left) {
                Interaction::Clicked
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
        (&Interaction, &Children, &mut Sprite, Option<&Secondary>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, mut sprite, secondary) in &mut interaction_query {
        if secondary.is_some() {
            if let Some(mut text) = children
                .get(0)
                .and_then(|child| text_query.get_mut(*child).ok())
            {
                text.sections[0].style.color = match *interaction {
                    Interaction::Clicked => SECONDARY_PRESSED_BUTTON_TEXT.into(),
                    Interaction::Hovered | Interaction::None => SECONDARY_BUTTON_TEXT.into(),
                };
            }

            *sprite = match *interaction {
                Interaction::Hovered => Sprite::from_color(SECONDARY_HOVERED_BUTTON),
                Interaction::Clicked | Interaction::None => Sprite::from_color(SECONDARY_BUTTON),
            };
        } else {
            *sprite = match *interaction {
                Interaction::Clicked => Sprite::from_color(PRESSED_BUTTON),
                Interaction::Hovered => Sprite::from_color(HOVERED_BUTTON),
                Interaction::None => Sprite::from_color(NORMAL_BUTTON),
            };
        }
    }
}
