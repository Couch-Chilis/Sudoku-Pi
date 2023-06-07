use super::{Note, Number, ScreenState, Selection};
use crate::{constants::*, sudoku::*, ui::*, utils::SpriteExt, Fonts, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::num::NonZeroU8;

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
    board.with_children(|board| {
        for x in 0..9 {
            board
                .spawn(FlexBundle::new(
                    FlexContainerStyle::default(),
                    FlexItemStyle::available_size(),
                ))
                .with_children(|column| {
                    for y in 0..9 {
                        spawn_cell(column, fonts, game, settings, x, y);
                    }
                });
        }
    });
}

fn spawn_cell(
    parent: &mut ChildBuilder,
    fonts: &Fonts,
    game: &Game,
    settings: &Settings,
    x: u8,
    y: u8,
) {
    let n = game.current.get(x, y);

    let number_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 70.,
        color: if n.is_some() {
            get_number_color(game, settings, x, y)
        } else {
            Color::NONE
        },
    };

    let note_style = TextStyle {
        font: fonts.bold.clone(),
        font_size: 30.,
        color: Color::NONE,
    };

    parent
        .spawn((
            Number(x, y),
            Interaction::None,
            FlexBundle::new(FlexContainerStyle::row(), FlexItemStyle::available_size()),
        ))
        .with_children(|cell| {
            cell.spawn(build_number(x, y, n, number_style));

            for note_x in 1..=3 {
                cell.spawn(FlexBundle::new(
                    FlexContainerStyle::default(),
                    FlexItemStyle::available_size(),
                ))
                .with_children(|note_column| {
                    for note_y in 0..3 {
                        note_column
                            .spawn(FlexBundle::new(
                                FlexContainerStyle::default(),
                                FlexItemStyle::available_size(),
                            ))
                            .with_children(|note_cell| {
                                let n = NonZeroU8::new(note_x + 3 * note_y).unwrap();
                                note_cell.spawn(build_note(x, y, n, note_style.clone()));
                            });
                    }
                });
            }
        });
}

fn build_number(x: u8, y: u8, cell: Cell, number_style: TextStyle) -> impl Bundle {
    (
        Number(x, y),
        FlexTextBundle::from_text(Text::from_section(
            cell.map(|n| n.to_string()).unwrap_or_default(),
            number_style,
        )),
    )
}

fn build_note(x: u8, y: u8, n: NonZeroU8, note_style: TextStyle) -> impl Bundle {
    (
        Note(x, y, n),
        FlexTextBundle::from_text(Text::from_section(n.to_string(), note_style)),
    )
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
    mut cells: Query<(&Interaction, &Number, &mut Sprite)>,
    screen: Res<State<ScreenState>>,
    game: Res<Game>,
    settings: Res<Settings>,
    selection: Res<Selection>,
) {
    if !game.is_changed() && !selection.is_changed() && !settings.is_changed() {
        return;
    }

    if screen.0 != ScreenState::Game && screen.0 != ScreenState::Highscores {
        return;
    }

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

    for (_interaction, number, mut sprite) in &mut cells {
        let pos = get_pos(number.0, number.1);
        let highlight_kind = highlights[pos];
        let color = match highlight_kind {
            Some(HighlightKind::Selection) => Color::rgba(0.9, 0.8, 0.0, 0.7),
            Some(HighlightKind::SameNumber) => Color::rgba(0.9, 0.8, 0.0, 0.45),
            Some(HighlightKind::InRange) => Color::rgba(0.9, 0.8, 0.0, 0.2),
            Some(HighlightKind::Note) => Color::rgba(0.9, 0.8, 0.0, 0.3),
            Some(HighlightKind::Hint) => COLOR_HINT,
            None => Color::NONE,
        };
        *sprite = Sprite::from_color(color);
    }
}
