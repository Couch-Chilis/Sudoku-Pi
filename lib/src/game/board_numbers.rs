use super::{Note, NoteAnimationKind, Number, ScreenState, Selection};
use crate::{
    constants::*,
    sudoku::*,
    ui::*,
    utils::{SpriteExt, TransformExt},
    Fonts, Settings,
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::num::NonZeroU8;

const NUMBER_FONT_SIZE: f32 = 70.;
const NOTE_FONT_SIZE: f32 = 30.;

#[derive(Component)]
pub(super) struct HighlightedNumber(usize, HighlightKind);

#[derive(Clone, Copy)]
pub(super) enum HighlightKind {
    Selection,
    SameNumber,
    InRange,
    Hint,
    Note,
    Mistake,
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
        font: if n.map(|n| game.is_completed(n)).unwrap_or_default() {
            fonts.light.clone()
        } else {
            fonts.bold.clone()
        },
        font_size: NUMBER_FONT_SIZE,
        color: if n.is_some() {
            get_number_color(game, settings, x, y)
        } else {
            Color::NONE
        },
    };

    let note_style = TextStyle {
        font: fonts.bold.clone(),
        font_size: NOTE_FONT_SIZE,
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
                        let n = NonZeroU8::new(note_x + 3 * note_y).unwrap();
                        note_column
                            .spawn((
                                Note::new(x, y, n),
                                FlexBundle::new(
                                    FlexContainerStyle::default(),
                                    FlexItemStyle::available_size(),
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
        ))
        .with_translation(0., -1.),
    )
}

fn build_note(x: u8, y: u8, n: NonZeroU8, note_style: TextStyle) -> impl Bundle {
    (
        Note::new(x, y, n),
        FlexTextBundle::from_text(Text::from_section(n.to_string(), note_style))
            .with_translation(0., 1.),
    )
}

pub(super) fn render_numbers(
    mut numbers: Query<(&Number, &mut Text)>,
    fonts: Res<Fonts>,
    game: Res<Game>,
    settings: Res<Settings>,
) {
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
                COLOR_MAIN_POP_DARK
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
    highlights: [Option<HighlightKind>; 81],
    selected_number: Option<NonZeroU8>,
}

impl Default for Highlights {
    fn default() -> Self {
        Self {
            highlights: [None; 81],
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

    let mut highlights = [None; 81];
    if let Some((x, y)) = selection.selected_cell {
        let selected_pos = get_pos(x, y);

        let selected_cell = game.current.get_by_pos(selected_pos);
        if let Some(n) = selected_cell {
            // Find all the cells with notes or mistakes containing the same number.
            for (pos, highlight) in highlights.iter_mut().enumerate() {
                let (x, y) = get_x_and_y_from_pos(pos);
                if settings.show_mistakes && game.mistakes.has(x, y, n) {
                    *highlight = Some(HighlightKind::Mistake);
                } else if game.notes.has(x, y, n) {
                    *highlight = Some(HighlightKind::Note);
                }
            }

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

    *highlights_resource = Highlights {
        highlights,
        selected_number: selection
            .selected_cell
            .and_then(|(x, y)| game.current.get(x, y)),
    };
}

pub(super) fn render_highlights(
    mut cells: Query<(&Number, &mut Sprite), Without<Note>>,
    mut notes: Query<(&mut Note, &mut FlexItemStyle, &mut Sprite), Without<Number>>,
    screen: Res<State<ScreenState>>,
    highlights: Res<Highlights>,
    time: Res<Time>,
) {
    if screen.get() != &ScreenState::Game && screen.get() != &ScreenState::Highscores {
        return;
    }

    if highlights.is_changed() {
        for (number, mut sprite) in &mut cells {
            let pos = get_pos(number.0, number.1);
            let highlight_kind = highlights.highlights[pos];
            let color = match highlight_kind {
                Some(HighlightKind::Selection) => Color::rgba(0.9, 0.8, 0.0, 0.7),
                Some(HighlightKind::SameNumber) => Color::rgba(0.9, 0.8, 0.0, 0.5),
                Some(HighlightKind::InRange) => Color::rgba(0.9, 0.8, 0.0, 0.2),
                Some(HighlightKind::Hint) => COLOR_HINT,
                None | Some(HighlightKind::Note) | Some(HighlightKind::Mistake) => Color::NONE,
            };
            if sprite.color != color {
                *sprite = Sprite::from_color(color);
            }
        }
    }

    for (note, flex_item_style, mut sprite) in &mut notes {
        let highlight_kind = if highlights
            .selected_number
            .map(|n| note.n == n)
            .unwrap_or_default()
        {
            highlights.highlights[get_pos(note.x, note.y)]
        } else {
            None
        };
        let color = match highlight_kind {
            Some(HighlightKind::Note) => Color::rgba(0.9, 0.8, 0.0, 0.5),
            Some(HighlightKind::Mistake) => COLOR_MAIN_POP_DARK.with_a(0.5),
            _ => Color::NONE,
        };
        if sprite.color != color {
            *sprite = Sprite::from_color(color);
        }

        if note.animation_kind == Some(NoteAnimationKind::Mistake) {
            animate_mistake(note, flex_item_style, time.delta().as_secs_f32());
        }
    }
}

fn animate_mistake(mut note: Mut<'_, Note>, mut style: Mut<'_, FlexItemStyle>, delta: f32) {
    let ratio = get_mistake_animation_ratios(note.animation_timer);

    style.transform = if ratio == 1. {
        note.animation_kind = None;
        Transform::default_2d()
    } else {
        note.animation_timer += delta;

        let font_ratio = NUMBER_FONT_SIZE / NOTE_FONT_SIZE;
        let zoom = 1. + (1. - ratio) * (font_ratio - 1.);
        let scale = Vec3::new(zoom, zoom, 1.);

        let (translate_x, translate_y) = match note.n.get() {
            1 => (font_ratio, -0.6),
            2 => (0., -0.6),
            3 => (-font_ratio, -0.6),
            4 => (font_ratio, 0.2),
            5 => (0., 0.2),
            6 => (-font_ratio, 0.2),
            7 => (font_ratio, 0.9),
            8 => (0., 0.9),
            9 => (-font_ratio, 0.9),
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

/// Returns the animation's ratio as a number from 0.0 through 1.0, where 0.0
/// means the animation hasn't started yet and 1.0 means it's done.
fn get_mistake_animation_ratios(timer: f32) -> f32 {
    const MISTAKE_ANIMATION_DELAY: f32 = 0.5;
    const MISTAKE_ANIMATION_DURATION: f32 = 0.5;

    if timer < MISTAKE_ANIMATION_DELAY {
        0.
    } else if timer > MISTAKE_ANIMATION_DELAY + MISTAKE_ANIMATION_DURATION {
        1.
    } else {
        ((timer - MISTAKE_ANIMATION_DELAY) / MISTAKE_ANIMATION_DURATION).powi(2)
    }
}
