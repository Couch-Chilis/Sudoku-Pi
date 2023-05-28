use super::{Board, Note, Number, ScreenState, Selection};
use crate::{constants::*, sudoku::*, Fonts, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::num::NonZeroU8;

// Font sizes are given with high values, to make sure we render the font at a
// high-enough resolution, then we scale back down to fit the squares.
const CELL_FONT_SIZE: f32 = 0.01667 * CELL_SIZE;
const FONT_SCALE: Vec3 = Vec3::new(CELL_FONT_SIZE, CELL_FONT_SIZE, 1.);

#[derive(Component)]
pub(super) struct HighlightedNumber(usize, HighlightKind);

#[derive(Clone, Copy)]
pub(super) enum HighlightKind {
    Selection,
    SameNumber,
    InRange,
    Hint,
    Note,
}

pub fn fill_numbers(board: &mut EntityCommands, fonts: &Fonts, game: &Game, settings: &Settings) {
    let number_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: Color::NONE,
    };

    let note_style = TextStyle {
        font: fonts.bold.clone(),
        font_size: 20.,
        color: Color::NONE,
    };

    board.with_children(|parent| {
        for x in 0..9 {
            for y in 0..9 {
                let cell = game.current.get(x, y);

                let mut style = number_style.clone();
                if cell.is_some() {
                    style.color = get_number_color(game, settings, x, y);
                }

                parent.spawn(build_number(x, y, cell, style));

                for n in 1..=9 {
                    let n = NonZeroU8::new(n).unwrap();
                    parent.spawn(build_note(x, y, n, note_style.clone()));
                }
            }
        }
    });
}

fn build_number(x: u8, y: u8, cell: Cell, number_style: TextStyle) -> impl Bundle {
    (
        Number(x, y),
        Text2dBundle {
            text: Text::from_section(
                cell.map(|n| n.to_string()).unwrap_or_default(),
                number_style,
            ),
            transform: Transform::from_translation(Vec3::new(
                (x as f32 - 4.) * CELL_SIZE,
                (y as f32 - 4.2) * CELL_SIZE,
                2.,
            ))
            .with_scale(FONT_SCALE),
            ..default()
        },
    )
}

fn build_note(x: u8, y: u8, n: NonZeroU8, note_style: TextStyle) -> impl Bundle {
    let (note_x, note_y) = get_note_coordinates(n);

    (
        Note(x, y, n),
        Text2dBundle {
            text: Text::from_section(n.to_string(), note_style),
            transform: Transform::from_translation(Vec3::new(
                ((x as f32 - 4.) + note_x) * CELL_SIZE,
                ((y as f32 - 4.06) + note_y) * CELL_SIZE,
                1.,
            ))
            .with_scale(FONT_SCALE),
            ..default()
        },
    )
}

fn get_note_coordinates(n: NonZeroU8) -> (f32, f32) {
    let x = match n.get() {
        1 | 4 | 7 => -0.3,
        2 | 5 | 8 => 0.,
        _ => 0.3,
    };

    let y = match n.get() {
        1 | 2 | 3 => 0.3,
        4 | 5 | 6 => 0.,
        _ => -0.3,
    };

    (x, y)
}

pub(super) fn render_numbers(
    mut numbers: Query<(&Number, &mut Text)>,
    game: Res<Game>,
    settings: Res<Settings>,
) {
    if !game.is_changed() && !settings.is_changed() {
        return;
    }

    for (Number(x, y), mut text) in &mut numbers {
        if let Some(n) = game.current.get(*x, *y) {
            text.sections[0].value = n.to_string();
            text.sections[0].style.color = get_number_color(&game, &settings, *x, *y);
        } else {
            text.sections[0].style.color = Color::NONE;
        };
    }
}

fn get_number_color(game: &Game, settings: &Settings, x: u8, y: u8) -> Color {
    if settings.show_mistakes {
        // If we show mistakes, there's no reason to visually differentiate
        // between starting numbers and numbers filled in correctly.
        if game.current.get(x, y) != game.solution.get(x, y) {
            COLOR_MAIN_POP_DARK
        } else {
            Color::BLACK
        }
    } else if game.start.has(x, y) {
        Color::BLACK
    } else {
        Color::BLUE
    }
}

pub(super) fn render_notes(mut notes: Query<(&Note, &mut Text)>, game: Res<Game>) {
    if !game.is_changed() {
        return;
    }

    for (Note(x, y, n), mut text) in &mut notes {
        text.sections[0].style.color = if game.notes.has(*x, *y, *n) && !game.current.has(*x, *y) {
            Color::BLACK
        } else {
            Color::NONE
        };
    }
}

pub(super) fn render_highlights(
    mut commands: Commands,
    screen: Res<State<ScreenState>>,
    game: Res<Game>,
    settings: Res<Settings>,
    selection: Res<Selection>,
    board: Query<Entity, With<Board>>,
    highlighted_numbers: Query<Entity, With<HighlightedNumber>>,
) {
    if !game.is_changed() && !selection.is_changed() && !settings.is_changed() {
        return;
    }

    if screen.0 != ScreenState::Game && screen.0 != ScreenState::Highscores {
        return;
    }

    for entity in &highlighted_numbers {
        commands.entity(entity).despawn();
    }

    let Ok(mut board) = board.get_single().map(|board| commands.entity(board)) else {
        return;
    };

    // First determine the type of highlight each cell should receive:
    let mut highlights = [None; 81];
    if let Some((x, y)) = selection.selected_cell {
        let selected_pos = get_pos(x, y);

        let selected_cell = game.current.get(x, y);
        if let Some(n) = selected_cell {
            if settings.highlight_selection_lines {
                // Find all the cells within range.
                for pos in 0..81 {
                    if game.current.get_by_pos(pos) == selected_cell {
                        let (x, y) = get_x_and_y_from_pos(pos);
                        for i in 0..9 {
                            highlights[get_pos(x, i)] = Some(HighlightKind::InRange);
                            highlights[get_pos(i, y)] = Some(HighlightKind::InRange);
                        }
                    }
                }
            } else {
                // Find all the cells with notes containing the same number.
                for (pos, highlight) in highlights.iter_mut().enumerate() {
                    let (x, y) = get_x_and_y_from_pos(pos);
                    if game.notes.has(x, y, n) {
                        *highlight = Some(HighlightKind::Note);
                    }
                }
            }

            // Find all the cells with the same number.
            for (pos, highlight) in highlights.iter_mut().enumerate() {
                if game.current.get_by_pos(pos) == selected_cell {
                    *highlight = Some(HighlightKind::SameNumber);
                }
            }
        }

        if !game.is_solved() {
            highlights[selected_pos] = Some(HighlightKind::Selection);
        }
    }
    if let Some((x, y)) = selection.hint {
        highlights[get_pos(x, y)] = Some(HighlightKind::Hint);
    }

    board.with_children(|parent| {
        for (pos, highlight) in highlights.into_iter().enumerate() {
            if let Some(highlight_kind) = highlight {
                let (x, y) = get_x_and_y_from_pos(pos);
                let color = match highlight_kind {
                    HighlightKind::Selection => Color::rgba(0.9, 0.8, 0.0, 0.7),
                    HighlightKind::SameNumber => Color::rgba(0.9, 0.8, 0.0, 0.45),
                    HighlightKind::InRange => Color::rgba(0.9, 0.8, 0.0, 0.2),
                    HighlightKind::Note => Color::rgba(0.9, 0.8, 0.0, 0.3),
                    HighlightKind::Hint => COLOR_HINT,
                };

                parent.spawn((
                    HighlightedNumber(pos, highlight_kind),
                    SpriteBundle {
                        sprite: Sprite { color, ..default() },
                        transform: get_cell_transform(x, y).with_scale(CELL_SCALE),
                        ..default()
                    },
                ));
            }
        }
    });
}

fn get_cell_transform(x: u8, y: u8) -> Transform {
    Transform::from_translation(Vec3::new(
        (x as f32 - 4.) * CELL_SIZE,
        (y as f32 - 4.) * CELL_SIZE,
        1.,
    ))
}
