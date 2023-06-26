use super::{Button, ButtonBackground, ButtonType, ComputedPosition};
use crate::{constants::*, utils::SpriteExt, ScreenState, ZoomFactor};
use bevy::{prelude::*, window::PrimaryWindow};

pub type InteractionEntity<'a> = (
    Entity,
    &'a mut Interaction,
    &'a ComputedPosition,
    &'a ComputedVisibility,
);
pub type InteractionQuery<'w, 's, 'a> = Query<'w, 's, InteractionEntity<'a>>;

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Selected,
    Pressed,
}

pub fn keyboard_interaction(
    mut interaction_query: InteractionQuery,
    screen: Res<State<ScreenState>>,
    keys: Res<Input<KeyCode>>,
) {
    if screen.0 == ScreenState::Game {
        return; // Game screen has its own controls.
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Up | Right | Down | Left => move_selection(&mut interaction_query, &screen.0, *key),
            Return => confirm_selection(&mut interaction_query, &screen.0),
            _ => {}
        }
    }
}

fn move_selection(interaction_query: &mut InteractionQuery, screen: &ScreenState, key: KeyCode) {
    let mut screen_entities: Vec<_> = interaction_query
        .iter_mut()
        .filter(|(_, _, computed_position, computed_visibility)| {
            computed_position.screens.contains(screen) && computed_visibility.is_visible()
        })
        .collect();

    let Some(selected_entity) = screen_entities
            .iter()
            .find(|(_, interaction, ..)| **interaction == Interaction::Selected) else {
                // Select the first interactive entity if there was no selection.
                if let Some((_, ref mut interaction, ..)) = screen_entities.first_mut() {
                    **interaction = Interaction::Selected;
                }
                return;
            };

    let mut candidates: Vec<_> = screen_entities
        .iter()
        .filter(|(_, _, computed_position, _)| {
            is_in_direction(computed_position, selected_entity.2, key)
        })
        .collect();
    candidates.sort_by_key(|(_, _, computed_position, _)| {
        get_distance(computed_position, selected_entity.2)
    });
    if let Some((nearest_entity, ..)) = candidates.first() {
        let nearest_entity = *nearest_entity;
        for (entity, ref mut interaction, ..) in screen_entities.iter_mut() {
            **interaction = if *entity == nearest_entity {
                Interaction::Selected
            } else {
                Interaction::None
            };
        }
    }
}

fn get_distance(position: &ComputedPosition, origin: &ComputedPosition) -> i32 {
    let position = position.center();
    let origin = origin.center();

    ((position.x - origin.x).powi(2) + (position.y - origin.y).powi(2)) as i32
}

fn is_in_direction(position: &ComputedPosition, origin: &ComputedPosition, key: KeyCode) -> bool {
    let position = position.center();
    let origin = origin.center();

    use KeyCode::*;
    match key {
        Up => {
            let dy = position.y - origin.y;
            dy > 0. && dy >= (position.x - origin.x).abs()
        }
        Right => {
            let dx = position.x - origin.x;
            dx > 0. && dx >= (position.y - origin.y).abs()
        }
        Down => {
            let dy = origin.y - position.y;
            dy > 0. && dy >= (position.x - origin.x).abs()
        }
        Left => {
            let dx = origin.x - position.x;
            dx > 0. && dx >= (position.y - origin.y).abs()
        }
        _ => false,
    }
}

fn confirm_selection(interaction_query: &mut InteractionQuery, screen: &ScreenState) {
    if let Some(mut selected_entity) = interaction_query.iter_mut().find(
        |(_, interaction, computed_position, computed_visibility)| {
            computed_position.screens.contains(screen)
                && computed_visibility.is_visible()
                && **interaction == Interaction::Selected
        },
    ) {
        *selected_entity.1 = Interaction::Pressed;
    }
}

pub fn mouse_interaction(
    mut interaction_query: InteractionQuery,
    mouse_buttons: Res<Input<MouseButton>>,
    screen: Res<State<ScreenState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    window_changes: Query<(With<PrimaryWindow>, Changed<Window>)>,
) {
    if !mouse_buttons.is_changed() && window_changes.is_empty() {
        return;
    }

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

pub fn touch_interaction(
    mut interaction_query: InteractionQuery,
    touches: Res<Touches>,
    screen: Res<State<ScreenState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    zoom_factor: Res<ZoomFactor>,
) {
    if !touches.is_changed() {
        return;
    }

    let Some(mut touch_position) = touches.first_pressed_position() else {
        return;
    };

    let Ok(window) = window_query.get_single() else {
        return;
    };

    touch_position.x *= zoom_factor.x;
    touch_position.y = window.height() - touch_position.y * zoom_factor.y;

    let selected_entity = interaction_query
        .iter()
        .find(|(_, _, computed_position, computed_visibility)| {
            computed_position.screens.contains(&screen.0)
                && computed_visibility.is_visible()
                && computed_position.contains(touch_position)
        })
        .map(|(entity, ..)| entity);

    for (entity, mut interaction, computed_position, _) in &mut interaction_query {
        let new_interaction = match selected_entity {
            Some(selected_entity) => {
                if selected_entity == entity {
                    if touches.any_just_pressed() {
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
