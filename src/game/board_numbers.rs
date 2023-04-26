use std::num::NonZeroU8;

use super::{Note, Number, OnGameScreen};
use crate::WindowSize;
use bevy::prelude::*;

pub fn fill_numbers(commands: &mut Commands, asset_server: &AssetServer, window_size: &WindowSize) {
    let font = asset_server.load("OpenSans-Regular.ttf");
    let square_size = 10. * window_size.vmin_scale;

    let mut numbers = Vec::with_capacity(81);
    let mut notes = Vec::with_capacity(9 * 81);

    for x in 0..9 {
        for y in 0..9 {
            numbers.push(build_number(font.clone(), square_size, x, y));

            for n in 1..=9 {
                let n = NonZeroU8::new(n).unwrap();
                notes.push(build_note(font.clone(), square_size, x, y, n));
            }
        }
    }

    commands.spawn_batch(numbers);
    commands.spawn_batch(notes);
}

fn build_number(font: Handle<Font>, square_size: f32, x: u8, y: u8) -> impl Bundle {
    (
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font,
                    font_size: 0.6 * square_size,
                    color: Color::NONE,
                },
            ),
            transform: Transform {
                translation: Vec3::new(
                    (x as f32 - 4.) * square_size,
                    (y as f32 - 4.) * square_size,
                    2.,
                ),
                ..default()
            },

            ..default()
        },
        Number(x, y),
        OnGameScreen,
    )
}

fn build_note(font: Handle<Font>, square_size: f32, x: u8, y: u8, n: NonZeroU8) -> impl Bundle {
    let (note_x, note_y) = get_note_coordinates(n);

    (
        Text2dBundle {
            text: Text::from_section(
                n.to_string(),
                TextStyle {
                    font,
                    font_size: 0.3 * square_size,
                    color: Color::NONE,
                },
            ),
            transform: Transform {
                translation: Vec3::new(
                    ((x as f32 - 4.) + note_x) * square_size,
                    ((y as f32 - 4.) + note_y) * square_size,
                    2.,
                ),
                ..default()
            },

            ..default()
        },
        Note(x, y, n),
        OnGameScreen,
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
