use crate::{constants::*, utils::*};
use anyhow::Context;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Default, Deserialize, Resource, Serialize)]
pub struct Highscores {
    pub best_scores: Vec<u32>,
    pub best_times: Vec<f32>,
}

impl Highscores {
    /// Adds a score and time to the highscores.
    ///
    /// Does nothing if the score doesn't reach the highscores.
    pub fn add(&mut self, new_score: u32, new_time: f32) {
        self.add_score(new_score);
        self.add_time(new_time);
    }

    fn add_score(&mut self, new_score: u32) {
        if let Some(index) = self.best_scores.iter().position(|score| new_score > *score) {
            self.best_scores.insert(index, new_score);
            self.best_scores.truncate(MAX_NUM_HIGHSCORES);
        } else if self.best_scores.len() < MAX_NUM_HIGHSCORES {
            self.best_scores.push(new_score);
        }
    }

    fn add_time(&mut self, new_time: f32) {
        if let Some(index) = self.best_times.iter().position(|time| new_time < *time) {
            self.best_times.insert(index, new_time);
            self.best_times.truncate(MAX_NUM_HIGHSCORES);
        } else if self.best_times.len() < MAX_NUM_HIGHSCORES {
            self.best_times.push(new_time);
        }
    }

    /// Loads highscores from disk, or returns `Self::default()` if no
    /// highscores could be loaded.
    pub fn load() -> Self {
        fs::read(ensure_sudoku_dir().join("highscores.json"))
            .context("Can't read file")
            .and_then(|json| Self::from_json(&json))
            .map_err(|err| println!("Can't load highscores: {err}"))
            .unwrap_or_default()
    }

    /// Saves highscores to disk.
    ///
    /// This is called automatically on drop.
    fn save(&self) {
        self.to_json()
            .and_then(|json| {
                fs::write(ensure_sudoku_dir().join("highscores.json"), json)
                    .context("Can't write to file")
            })
            .unwrap_or_else(|err| println!("Can't save highscores: {err}"));
    }

    /// Serializes the highscores to JSON.
    fn to_json(&self) -> Result<Vec<u8>, anyhow::Error> {
        serde_json::to_vec(self).map_err(anyhow::Error::from)
    }

    /// Parses highscores from JSON.
    fn from_json(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        serde_json::from_slice(bytes).map_err(anyhow::Error::from)
    }
}

impl Drop for Highscores {
    fn drop(&mut self) {
        self.save()
    }
}
