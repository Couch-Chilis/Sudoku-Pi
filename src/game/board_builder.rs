use super::{board_line_bundle::*, OnGameScreen};
use crate::WindowSize;
use bevy::prelude::*;

pub fn build_board(commands: &mut Commands, window_size: &WindowSize) {
    use Orientation::*;
    use Thickness::*;

    #[rustfmt::skip]
    commands.spawn_batch([
        (BoardLineBundle::new(window_size, 0, Horizontal, Thick), OnGameScreen),
        (BoardLineBundle::new(window_size, 1, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 2, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 3, Horizontal, Medium), OnGameScreen),
        (BoardLineBundle::new(window_size, 4, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 5, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 6, Horizontal, Medium), OnGameScreen),
        (BoardLineBundle::new(window_size, 7, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 8, Horizontal, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 9, Horizontal, Thick), OnGameScreen),
        (BoardLineBundle::new(window_size, 0, Vertical, Thick), OnGameScreen),
        (BoardLineBundle::new(window_size, 1, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 2, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 3, Vertical, Medium), OnGameScreen),
        (BoardLineBundle::new(window_size, 4, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 5, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 6, Vertical, Medium), OnGameScreen),
        (BoardLineBundle::new(window_size, 7, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 8, Vertical, Thin), OnGameScreen),
        (BoardLineBundle::new(window_size, 9, Vertical, Thick), OnGameScreen),
    ]);
}
