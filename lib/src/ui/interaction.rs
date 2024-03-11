use super::{Button, ButtonBackground, ButtonType, ComputedPosition};
use crate::{constants::*, game::Wheel, pointer_query::*, utils::SpriteExt, Screen, ScreenState};
use bevy::prelude::*;
use std::collections::BTreeMap;

pub type InteractionEntity<'a> = (
    Entity,
    &'a mut Interaction,
    &'a ComputedPosition,
    &'a ViewVisibility,
);
pub type InteractionQuery<'w, 's, 'a> = Query<'w, 's, InteractionEntity<'a>>;

#[derive(Clone, Component, Debug, Default)]
pub struct InitialSelection;

#[derive(Clone, Component, Copy, Debug, Default, Eq, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Selected,
    Pressed,
}

pub fn keyboard_interaction(
    mut interaction_query: InteractionQuery,
    screen: Res<State<ScreenState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if screen.get() == &ScreenState::Game {
        return; // Game screen has its own controls.
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp | ArrowRight | ArrowDown | ArrowLeft => {
                move_selection(&mut interaction_query, screen.get(), *key)
            }
            Enter => confirm_selection(&mut interaction_query, screen.get()),
            _ => {}
        }
    }
}

fn move_selection(interaction_query: &mut InteractionQuery, screen: &ScreenState, key: KeyCode) {
    let mut screen_entities: Vec<_> = interaction_query
        .iter_mut()
        .filter(|(_, _, computed_position, visibility)| {
            computed_position.screens.contains(screen) && visibility.get()
        })
        .collect();

    let Some(selected_entity) = screen_entities
        .iter()
        .find(|(_, interaction, ..)| **interaction == Interaction::Selected)
    else {
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
        ArrowUp => {
            let dy = position.y - origin.y;
            dy > 0. && dy >= (position.x - origin.x).abs()
        }
        ArrowRight => {
            let dx = position.x - origin.x;
            dx > 0. && dx >= (position.y - origin.y).abs()
        }
        ArrowDown => {
            let dy = origin.y - position.y;
            dy > 0. && dy >= (position.x - origin.x).abs()
        }
        ArrowLeft => {
            let dx = origin.x - position.x;
            dx > 0. && dx >= (position.y - origin.y).abs()
        }
        _ => false,
    }
}

fn confirm_selection(interaction_query: &mut InteractionQuery, screen: &ScreenState) {
    if let Some(mut selected_entity) =
        interaction_query
            .iter_mut()
            .find(|(_, interaction, computed_position, visibility)| {
                computed_position.screens.contains(screen)
                    && visibility.get()
                    && **interaction == Interaction::Selected
            })
    {
        *selected_entity.1 = Interaction::Pressed;
    }
}

pub fn pointer_interaction(
    mut interaction_query: InteractionQuery,
    screen: Res<State<ScreenState>>,
    pointer_query: PointerQuery,
    wheel_query: Query<(&Wheel, &ScreenState)>,
) {
    if wheel_query
        .iter()
        .find_map(|wheel| (wheel.1 == screen.get()).then_some(wheel.0.is_open))
        .unwrap_or_default()
    {
        return;
    }

    let Some((input_kind, position)) = pointer_query
        .get_changed_input_with_position()
        .map(|(input_kind, position)| (Some(input_kind), position))
        .or_else(|| {
            pointer_query
                .get_changed_hover()
                .map(|position| (None, position))
        })
    else {
        return;
    };

    let selected_entity = interaction_query
        .iter()
        .find(|(_, _, computed_position, visibility)| {
            computed_position.screens.contains(screen.get())
                && visibility.get()
                && computed_position.contains(position)
        })
        .map(|(entity, ..)| entity);

    for (entity, mut interaction, computed_position, _) in &mut interaction_query {
        let new_interaction = match selected_entity {
            Some(selected_entity) => {
                if selected_entity == entity {
                    if input_kind == Some(InputKind::Press) {
                        Interaction::Pressed
                    } else {
                        Interaction::Selected
                    }
                } else if computed_position.screens.contains(screen.get()) {
                    Interaction::None
                } else {
                    *interaction
                }
            }
            None => *interaction,
        };

        if *interaction != new_interaction {
            *interaction = new_interaction;
        }
    }
}

// Handles changing button color based on pointer interaction.
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
                    .first()
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
                    .first()
                    .and_then(|child| background_query.get_mut(*child).ok())
                {
                    *background = match *interaction {
                        Interaction::Pressed => {
                            Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND_PRESS)
                        }
                        _ => Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
                    };

                    if let Some(mut text) = children
                        .first()
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
                    .first()
                    .and_then(|child| background_query.get_mut(*child).ok())
                {
                    *background = match *interaction {
                        Interaction::Pressed => {
                            Sprite::from_color(COLOR_TERNARY_BUTTON_BACKGROUND_PRESS)
                        }
                        _ => Sprite::from_color(COLOR_TERNARY_BUTTON_BACKGROUND),
                    };

                    if let Some(mut text) = children
                        .first()
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

/// When navigating to a new screen, only the component that is marked with
/// `InitialSelection` should be initially selected.
pub(crate) fn reset_initial_selection_on_screen_change(
    mut interaction_query: Query<(Entity, &mut Interaction, Option<&InitialSelection>)>,
    screen_query: Query<(Entity, &Children, Option<&Screen>)>,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() {
        return;
    }

    let mut screen_entity = None;
    let mut children_map = BTreeMap::new();

    for (entity, children, screen) in &screen_query {
        children_map.insert(entity, children);

        if screen.is_some_and(|screen| &screen.state == screen_state.get()) {
            screen_entity = Some(entity);
        }
    }

    let Some(screen_entity) = screen_entity else {
        return;
    };

    for (entity, mut interaction, initial_selection) in &mut interaction_query {
        if !is_child_of(entity, screen_entity, &children_map) {
            continue;
        }

        let new_interaction = match initial_selection {
            Some(_) => Interaction::Selected,
            None => Interaction::None,
        };

        if *interaction != new_interaction {
            *interaction = new_interaction;
        }
    }
}

fn is_child_of(entity: Entity, parent: Entity, children_map: &BTreeMap<Entity, &Children>) -> bool {
    if let Some(children) = children_map.get(&parent) {
        children.contains(&entity)
            || children
                .iter()
                .any(|child| is_child_of(entity, *child, children_map))
    } else {
        false
    }
}
