use crate::settings::Settings;
use bevy::prelude::*;

#[derive(Clone, Component, Copy)]
pub enum SettingsToggle {
    HighlightSelectionLines,
    ShowMistakes,
}

impl SettingsToggle {
    pub fn is_enabled(&self, settings: &Settings) -> bool {
        match self {
            SettingsToggle::HighlightSelectionLines => settings.highlight_selection_lines,
            SettingsToggle::ShowMistakes => settings.show_mistakes,
        }
    }
}
