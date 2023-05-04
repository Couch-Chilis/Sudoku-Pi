use super::{get_number_color, Note, Number};
use crate::sudoku::{Cell, Game};
use crate::{constants::*, Fonts, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::num::NonZeroU8;

// Font sizes are given with high values, to make sure we render the font at a
// high-enough resolution, then we scale back down to fit the squares.
const CELL_FONT_SIZE: f32 = 0.01667 * CELL_SIZE;
const FONT_SCALE: Vec3 = Vec3::new(CELL_FONT_SIZE, CELL_FONT_SIZE, 1.);

pub fn fill_numbers(board: &mut EntityCommands, fonts: &Fonts, game: &Game, settings: &Settings) {
    let number_style = TextStyle {
        font: fonts.board.clone(),
        font_size: 60.,
        color: Color::NONE,
    };

    let note_style = TextStyle {
        font: fonts.board.clone(),
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
                (y as f32 - 4.) * CELL_SIZE,
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
                ((y as f32 - 4.) + note_y) * CELL_SIZE,
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
