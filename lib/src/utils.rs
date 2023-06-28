use bevy::prelude::*;
use std::{fs, path::PathBuf};

const DEFAULT_TRANSLATION: Vec3 = Vec3::new(0., 0., 1.);

pub trait SpriteExt {
    fn from_color(color: Color) -> Sprite;
}

impl SpriteExt for Sprite {
    fn from_color(color: Color) -> Sprite {
        Sprite { color, ..default() }
    }
}

pub trait TransformExt {
    fn default_2d() -> Transform;
    fn from_2d_scale(x: f32, y: f32) -> Transform;
}

impl TransformExt for Transform {
    fn default_2d() -> Transform {
        Transform {
            translation: DEFAULT_TRANSLATION,
            ..default()
        }
    }

    fn from_2d_scale(x: f32, y: f32) -> Transform {
        Transform {
            translation: DEFAULT_TRANSLATION,
            scale: Vec3::new(x, y, 1.),
            ..default()
        }
    }
}

pub fn ensure_sudoku_dir() -> PathBuf {
    if cfg!(target_os = "ios") {
        PathBuf::from("Library/Application support")
    } else {
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
}

pub fn format_time(time_secs: f32) -> String {
    let minutes = (time_secs / 60.).floor();
    let seconds = (time_secs - minutes * 60.).floor();
    format!("{minutes}:{seconds:02}")
}
