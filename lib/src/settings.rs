use crate::utils::ensure_sudoku_dir;
use anyhow::Context;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Resource, Serialize)]
pub struct Settings {
    #[serde(default)]
    pub highlight_selection_lines: bool,

    #[serde(default)]
    pub show_mistakes: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            highlight_selection_lines: false,
            show_mistakes: true,
        }
    }
}

impl Settings {
    /// Loads settings from disk, or returns `Self::default()` if no
    /// settings could be loaded.
    pub fn load() -> Self {
        fs::read(ensure_sudoku_dir().join("settings.json"))
            .context("Can't read file")
            .and_then(|json| Self::from_json(&json))
            .map_err(|err| println!("Can't load settings: {err}"))
            .unwrap_or_default()
    }

    /// Saves settings to disk.
    ///
    /// This is called automatically on drop.
    pub fn save(&self) {
        self.to_json()
            .and_then(|json| {
                fs::write(ensure_sudoku_dir().join("settings.json"), json).map_err(anyhow::Error::from)
            })
            .unwrap_or_else(|err| println!("Can't save settings: {err}"));
    }

    /// Serializes the settings to JSON.
    fn to_json(&self) -> Result<Vec<u8>, anyhow::Error> {
        serde_json::to_vec(self).map_err(anyhow::Error::from)
    }

    /// Parses settings from JSON.
    fn from_json(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        serde_json::from_slice(bytes).map_err(anyhow::Error::from)
    }
}
