use bevy::prelude::*;
use std::{fs, path::PathBuf};

pub trait SpriteExt {
    fn from_color(color: Color) -> Sprite;
}

impl SpriteExt for Sprite {
    fn from_color(color: Color) -> Sprite {
        Sprite { color, ..default() }
    }
}

pub fn ensure_sudoku_dir() -> PathBuf {
    #[allow(deprecated)]
    let parent_dir = std::env::home_dir().unwrap_or(PathBuf::from("/tmp"));
    let sudoku_dir = parent_dir.join(".sudoku");
    if sudoku_dir.exists() {
        return sudoku_dir;
    }

    match fs::create_dir_all(&sudoku_dir) {
        Ok(()) => sudoku_dir,
        Err(_) => parent_dir,
    }
}
