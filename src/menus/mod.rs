mod button_builder;
mod how_to_play;
mod main_menu;
mod score;
mod settings_menu;
mod toggle_builder;

use crate::ScreenState;
use bevy::prelude::*;
use button_builder::ButtonBuilder;
use main_menu::{difficulty_button_actions, main_button_actions, on_screen_change};
//use toggle_builder::ToggleBuilder;

pub use main_menu::main_menu_setup;
//pub use settings_menu::settings_menu_setup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(difficulty_button_actions.run_if(in_state(ScreenState::SelectDifficulty)))
            .add_system(main_button_actions.run_if(in_state(ScreenState::MainMenu)))
            .add_system(on_screen_change);
    }
}
