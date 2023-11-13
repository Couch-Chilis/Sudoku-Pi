use super::{MistakeCellBorders, Note, NoteAnimationKind, Number, Selection};
use crate::{
    constants::*,
    resource_bag::ResourceBag,
    sudoku::*,
    ui::*,
    utils::{SpriteExt, TransformExt},
    Fonts, ScreenSizing, Settings,
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::num::NonZeroU8;

const NUMBER_FONT_SIZE: f32 = 80.;
const NUMBER_FONT_SIZE_IPAD: f32 = 100.;
const NOTE_FONT_SIZE: f32 = 30.;
const NOTE_FONT_SIZE_IPAD: f32 = 40.;

#[derive(Clone, Copy)]
pub(super) enum CellHighlightKind {
    Selection,
    SameNumber,
    InRange,
    Hint,
}

#[derive(Clone, Copy)]
pub(super) enum NoteHighlightKind {
    Note,
    Mistake,
}

pub fn fill_numbers(
    board: &mut EntityCommands,
    game: &Game,
    resources: &ResourceBag,
    settings: &Settings,
) {
    board.with_children(|board| {
        for x in 0..9 {
            board
                .spawn(FlexBundle::new(
                    FlexItemStyle::available_size(),
                    FlexContainerStyle::column(),
                ))
                .with_children(|column| {
                    for y in 0..9 {
                        spawn_cell(column, game, resources, settings, x, y);
                    }
                });
        }
    });
}

fn spawn_cell(
    parent: &mut ChildBuilder,
    game: &Game,
    resources: &ResourceBag,
    settings: &Settings,
    x: u8,
    y: u8,
) {
    let n = game.current.get(x, y);

    let number_style = TextStyle {
        font: if n.map(|n| game.is_completed(n)).unwrap_or_default() {
            resources.fonts.light.clone()
        } else {
            resources.fonts.bold.clone()
        },
        font_size: if resources.screen_sizing.is_ipad {
            NUMBER_FONT_SIZE_IPAD
        } else {
            NUMBER_FONT_SIZE
        },
        color: if n.is_some() {
            get_number_color(game, settings, x, y)
        } else {
            Color::NONE
        },
    };

    let note_style = TextStyle {
        font: resources.fonts.bold.clone(),
        font_size: if resources.screen_sizing.is_ipad {
            NOTE_FONT_SIZE_IPAD
        } else {
            NOTE_FONT_SIZE
        },
        color: Color::NONE,
    };

    parent
        .spawn((
            Number(x, y),
            Interaction::None,
            FlexBundle::new(FlexItemStyle::available_size(), FlexContainerStyle::row()),
        ))
        .with_children(|cell| {
            cell.spawn(build_number(x, y, n, number_style));

            for note_x in 1..=3 {
                cell.spawn(FlexBundle::new(
                    FlexItemStyle::available_size(),
                    FlexContainerStyle::default(),
                ))
                .with_children(|note_column| {
                    for note_y in 0..3 {
                        let n = NonZeroU8::new(note_x + 3 * note_y).unwrap();
                        note_column
                            .spawn((
                                Note::new(x, y, n),
                                FlexBundle::new(
                                    FlexItemStyle::available_size(),
                                    FlexContainerStyle::default(),
                                ),
                            ))
                            .with_children(|note_cell| {
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
        Note::new(x, y, n),
        FlexTextBundle::from_text(Text::from_section(n.to_string(), note_style)),
    )
}

pub(super) fn render_numbers(
    mut numbers: Query<(&Number, &mut Text)>,
    fonts: Res<Fonts>,
    game: Res<Game>,
    settings: Res<Settings>,
) {
    if !game.is_changed() && !settings.is_changed() {
        return;
    }

    for (Number(x, y), mut text) in &mut numbers {
        let current_color = text.sections[0].style.color;
        let new_color = if let Some(n) = game.current.get(*x, *y) {
            text.sections[0].value = n.to_string();
            text.sections[0].style.font = if game.is_completed(n) {
                fonts.light.clone()
            } else {
                fonts.bold.clone()
            };
            get_number_color(&game, &settings, *x, *y)
        } else {
            Color::NONE
        };

        if new_color != current_color {
            text.sections[0].style.color = new_color;
        }
    }
}

fn get_number_color(game: &Game, settings: &Settings, x: u8, y: u8) -> Color {
    if settings.show_mistakes {
        // If we show mistakes, there's no reason to visually differentiate
        // between starting numbers and numbers filled in correctly.
        if game.current.get(x, y) != game.solution.get(x, y) {
            COLOR_POP_DARK
        } else {
            Color::BLACK
        }
    } else if game.start.has(x, y) {
        Color::BLACK
    } else {
        Color::BLUE
    }
}

pub(super) fn render_notes(
    mut notes: Query<(&mut Note, &mut Text)>,
    game: Res<Game>,
    settings: Res<Settings>,
    time: Res<Time>,
) {
    for (mut note, mut text) in &mut notes {
        let x = note.x;
        let y = note.y;
        let n = note.n;

        let current_color = text.sections[0].style.color;
        let new_color =
            if settings.show_mistakes && game.mistakes.has(x, y, n) && !game.current.has(x, y) {
                COLOR_POP_DARK
            } else if note.animation_kind == Some(NoteAnimationKind::MistakeInCell) {
                let (ratio, _) = get_mistake_animation_ratio(note.animation_timer);
                if ratio < 1. {
                    note.animation_timer += time.delta().as_secs_f32();
                    Color::rgba(0., 0., 0., ratio.powi(2))
                } else {
                    note.animation_kind = None;
                    Color::BLACK
                }
            } else if game.notes.has(x, y, n) && !game.current.has(x, y) {
                Color::BLACK
            } else if let Some(NoteAnimationKind::FadeOut(duration)) = note.animation_kind {
                let a = 1. - note.animation_timer / duration.as_secs_f32();
                if a <= 0. {
                    note.animation_kind = None;
                    Color::NONE
                } else {
                    note.animation_timer += time.delta().as_secs_f32();
                    Color::rgba(0., 0., 0., a)
                }
            } else {
                Color::NONE
            };

        if new_color != current_color {
            text.sections[0].style.color = new_color;
        }
    }
}

#[derive(Resource)]
pub(super) struct Highlights {
    cell_highlights: [Option<CellHighlightKind>; 81],
    note_highlights: [Option<NoteHighlightKind>; 81],
    selected_number: Option<NonZeroU8>,
}

impl Default for Highlights {
    fn default() -> Self {
        Self {
            cell_highlights: [None; 81],
            note_highlights: [None; 81],
            selected_number: None,
        }
    }
}

pub(super) fn calculate_highlights(
    mut highlights_resource: ResMut<Highlights>,
    game: Res<Game>,
    settings: Res<Settings>,
    selection: Res<Selection>,
) {
    if !game.is_changed() && !selection.is_changed() && !settings.is_changed() {
        return;
    }

    let selected_number = selection
        .selected_cell
        .and_then(|(x, y)| game.current.get(x, y))
        .or(selection.selected_note);

    let mut cell_highlights = [None; 81];
    let mut note_highlights = [None; 81];
    if let Some((x, y)) = selection.selected_cell {
        let selected_pos = get_pos(x, y);

        if let Some(n) = selected_number {
            // Find all the cells with notes or mistakes containing the same number.
            for (pos, highlight) in note_highlights.iter_mut().enumerate() {
                let (x, y) = get_x_and_y_from_pos(pos);
                if settings.show_mistakes && game.mistakes.has(x, y, n) {
                    *highlight = Some(NoteHighlightKind::Mistake);
                } else if game.notes.has(x, y, n) {
                    *highlight = Some(NoteHighlightKind::Note);
                }
            }

            let selected_cell = game.current.get_by_pos(selected_pos);
            if settings.selected_cell_highlight && selected_cell.is_some() {
                // Find all the cells within range.
                for pos in 0..81 {
                    if game.current.get_by_pos(pos) == selected_cell {
                        let (x, y) = get_x_and_y_from_pos(pos);
                        let block_x = get_block_offset(x);
                        let block_y = get_block_offset(y);

                        for i in 0..9 {
                            cell_highlights[get_pos(x, i)] = Some(CellHighlightKind::InRange);
                            cell_highlights[get_pos(i, y)] = Some(CellHighlightKind::InRange);
                            cell_highlights[get_pos(block_x + i / 3, block_y + i % 3)] =
                                Some(CellHighlightKind::InRange);
                        }
                    }
                }
            }

            // Find all the cells with the same number.
            for (pos, highlight) in cell_highlights.iter_mut().enumerate() {
                if game.current.get_by_pos(pos) == selected_number {
                    *highlight = Some(CellHighlightKind::SameNumber);
                }
            }
        }

        if !game.is_solved() && selection.selected_note.is_none() {
            cell_highlights[selected_pos] = Some(CellHighlightKind::Selection);
        }
    }
    if let Some((x, y)) = selection.hint {
        cell_highlights[get_pos(x, y)] = Some(CellHighlightKind::Hint);
    }

    *highlights_resource = Highlights {
        cell_highlights,
        note_highlights,
        selected_number,
    };
}

pub(super) fn render_cell_highlights(
    mut cells: Query<(&Number, &mut Sprite)>,
    highlights: Res<Highlights>,
) {
    if !highlights.is_changed() {
        return;
    }

    for (number, mut sprite) in &mut cells {
        let pos = get_pos(number.0, number.1);
        let highlight_kind = highlights.cell_highlights[pos];
        let color = match highlight_kind {
            Some(CellHighlightKind::Selection) => COLOR_CELL_SELECTION,
            Some(CellHighlightKind::SameNumber) => COLOR_CELL_SAME_NUMBER,
            Some(CellHighlightKind::InRange) => COLOR_CELL_HIGHLIGHT,
            Some(CellHighlightKind::Hint) => COLOR_HINT,
            None => Color::NONE,
        };
        if sprite.color != color {
            *sprite = Sprite::from_color(color);
        }
    }
}

pub(super) fn render_note_highlights(
    mut notes: Query<(&mut Note, &mut FlexItemStyle, &mut Sprite)>,
    mut mistake_borders: Query<(&mut Transform, &mut Visibility), With<MistakeCellBorders>>,
    screen_sizing: Res<ScreenSizing>,
    highlights: Res<Highlights>,
    time: Res<Time>,
) {
    for (note, flex_item_style, mut sprite) in &mut notes {
        let highlight_kind = if highlights.selected_number == Some(note.n) {
            highlights.note_highlights[get_pos(note.x, note.y)]
        } else {
            None
        };
        let color = match highlight_kind {
            Some(NoteHighlightKind::Note) => COLOR_CELL_SAME_NUMBER,
            Some(NoteHighlightKind::Mistake) => COLOR_POP_DARK.with_a(0.5),
            None => Color::NONE,
        };
        if sprite.color != color {
            *sprite = Sprite::from_color(color);
        }

        if note.animation_kind == Some(NoteAnimationKind::Mistake) {
            animate_mistake(
                note,
                &mut mistake_borders,
                flex_item_style,
                time.delta().as_secs_f32(),
                &screen_sizing,
            );
        }
    }
}

fn animate_mistake(
    mut note: Mut<'_, Note>,
    mistake_borders: &mut Query<(&mut Transform, &mut Visibility), With<MistakeCellBorders>>,
    mut style: Mut<'_, FlexItemStyle>,
    delta: f32,
    screen_sizing: &ScreenSizing,
) {
    let (ratio, show_borders) = get_mistake_animation_ratio(note.animation_timer);

    for (mut mistake_borders_transform, mut mistake_borders_visibility) in
        mistake_borders.iter_mut()
    {
        if show_borders {
            mistake_borders_transform.translation.x = (note.x as f32 - 4.) * CELL_SIZE;
            mistake_borders_transform.translation.y = -(note.y as f32 - 4.) * CELL_SIZE;
            *mistake_borders_visibility = Visibility::Visible;
        } else {
            *mistake_borders_visibility = Visibility::Hidden;
        }
    }

    style.transform = if ratio == 1. {
        note.animation_kind = None;
        Transform::default_2d()
    } else {
        note.animation_timer += delta;

        let font_ratio = if screen_sizing.is_ipad {
            NUMBER_FONT_SIZE_IPAD / NOTE_FONT_SIZE_IPAD
        } else {
            NUMBER_FONT_SIZE / NOTE_FONT_SIZE
        };
        let zoom = 1. + (1. - ratio) * (font_ratio - 1.);
        let scale = Vec3::new(zoom, zoom, 1.);

        let (translate_x, translate_y) = match note.n.get() {
            1 => (font_ratio, -0.9),
            2 => (0., -0.9),
            3 => (-font_ratio, -0.9),
            4 => (font_ratio, -0.05),
            5 => (0., -0.05),
            6 => (-font_ratio, -0.05),
            7 => (font_ratio, 0.85),
            8 => (0., 0.85),
            9 => (-font_ratio, 0.85),
            _ => (0., 0.),
        };

        let translation =
            Vec3::new(translate_x * (1. - ratio), translate_y * (1. - ratio), 0.) / scale;

        Transform {
            translation,
            scale,
            ..default()
        }
    };
}

/// Returns a tuple with the animation's ratio as a number from 0.0 through 1.0,
/// where 0.0 means the animation hasn't started yet and 1.0 means it's done.
///
/// The second value in the tuple determines whether the mistake borders should
/// be shown or not.
fn get_mistake_animation_ratio(timer: f32) -> (f32, bool) {
    const MISTAKE_ANIMATION_DELAY: f32 = 0.8;
    const MISTAKE_ANIMATION_DURATION: f32 = 0.5;

    if timer < MISTAKE_ANIMATION_DELAY {
        (
            0.,
            (0.0..0.2).contains(&timer)
                || (0.3..0.5).contains(&timer)
                || (0.6..0.8).contains(&timer),
        )
    } else if timer > MISTAKE_ANIMATION_DELAY + MISTAKE_ANIMATION_DURATION {
        (1., false)
    } else {
        (
            ((timer - MISTAKE_ANIMATION_DELAY) / MISTAKE_ANIMATION_DURATION).powi(2),
            false,
        )
    }
}
