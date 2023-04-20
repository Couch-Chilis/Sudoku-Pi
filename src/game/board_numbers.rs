use std::num::NonZeroU8;

use crate::sudoku::Game;
use crate::WindowSize;
use bevy::prelude::*;

use super::{Number, OnGameScreen};

pub fn fill_numbers(
    commands: &mut Commands,
    asset_server: &AssetServer,
    game: &Game,
    window_size: &WindowSize,
) {
    let font = asset_server.load("OpenSans-Regular.ttf");

    for x in 0..9 {
        for y in 0..9 {
            if let Some(n) = game.start.get(x, y) {
                spawn_number(commands, font.clone(), window_size, x, y, n);
            }
        }
    }
}

pub fn spawn_number(
    commands: &mut Commands,
    font: Handle<Font>,
    window_size: &WindowSize,
    x: u8,
    y: u8,
    n: NonZeroU8,
) {
    let scale = 10. * window_size.vmin_scale;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                n.to_string(),
                TextStyle {
                    font,
                    font_size: 6. * window_size.vmin_scale,
                    color: Color::BLACK,
                },
            ),
            transform: Transform {
                translation: Vec3::new((x as f32 - 4.) * scale, (y as f32 - 4.) * scale, 2.),
                ..default()
            },

            ..default()
        },
        Number,
        OnGameScreen,
    ));
}
