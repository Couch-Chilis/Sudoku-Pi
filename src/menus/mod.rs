mod button_builder;
mod how_to_play;
mod main_menu;
mod score;
mod toggle_builder;

use crate::ScreenState;
use bevy::prelude::*;
use button_builder::ButtonBuilder;
use main_menu::{
    difficulty_screen_button_actions, main_screen_button_actions, menu_button_actions,
    menu_interaction, on_screen_change,
};
use toggle_builder::ToggleBuilder;

pub use main_menu::main_menu_setup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            difficulty_screen_button_actions.run_if(in_state(ScreenState::SelectDifficulty)),
        )
        .add_system(main_screen_button_actions.run_if(in_state(ScreenState::MainMenu)))
        .add_system(menu_interaction.run_if(in_main_menu))
        .add_system(menu_button_actions.run_if(in_main_menu))
        .add_system(on_screen_change);
    }
}

fn in_main_menu(state: Res<State<ScreenState>>) -> bool {
    matches!(
        state.0,
        ScreenState::MainMenu | ScreenState::SelectDifficulty | ScreenState::Settings
    )
}
