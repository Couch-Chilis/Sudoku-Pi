use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::prelude::*;

const DEFAULT_TRANSLATION: Vec3 = Vec3::new(0., 0., 1.);

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
    let sudoku_dir = if cfg!(target_os = "android") {
        bevy::android::ANDROID_APP
            .get()
            .and_then(|app| app.internal_data_path())
            .unwrap_or_else(|| PathBuf::from("/data/local/tmp"))
    } else if cfg!(target_os = "ios") {
        let home = std::env::home_dir().unwrap_or(PathBuf::from("/tmp"));
        home.join("Library/Application support")
    } else {
        let home = std::env::home_dir().unwrap_or(PathBuf::from("/tmp"));
        home.join(".sudoku")
    };
    if sudoku_dir.exists() {
        return sudoku_dir;
    }

    match fs::create_dir_all(&sudoku_dir) {
        Ok(()) => sudoku_dir,
        Err(err) => {
            let parent_dir = sudoku_dir
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or(sudoku_dir);
            warn!("Falling back to parent dir ({parent_dir:?}): {err:?}");
            parent_dir
        }
    }
}

pub fn format_time(time_secs: f32) -> String {
    let minutes = (time_secs / 60.).floor();
    let seconds = (time_secs - minutes * 60.).floor();
    format!("{minutes}:{seconds:02}")
}
