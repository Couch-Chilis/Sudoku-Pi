mod button_builder;
mod how_to_play;
mod main_menu;
mod score;
mod settings;

use crate::sudoku::Game;
use crate::ui::{Button, Interaction};
use crate::ScreenState;
use bevy::{app::AppExit, prelude::*};
use button_builder::ButtonBuilder;
use main_menu::on_screen_change;

pub use main_menu::main_menu_setup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(difficulty_button_actions.run_if(in_state(ScreenState::SelectDifficulty)))
            .add_system(main_button_actions.run_if(in_state(ScreenState::MainMenu)))
            .add_system(on_screen_change);
    }
}

#[derive(Component)]
pub enum MainButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    //GoToOptions,
    Quit,
}

// Handles screen navigation based on button actions in the main screen.
fn main_button_actions(
    query: Query<(&Interaction, &MainButtonAction), (Changed<Interaction>, With<Button>)>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Clicked {
            match action {
                MainButtonAction::ContinueGame => screen_state.set(ScreenState::Game),
                MainButtonAction::GoToHowToPlay => screen_state.set(ScreenState::HowToPlay),
                MainButtonAction::GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                //MenuButtonAction::GoToOptions => screen_state.set(ScreenState::Options),
                MainButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

#[derive(Component)]
pub enum DifficultyButtonAction {
    BackToMain,
    StartGameAtDifficulty(u8),
}

// Handles screen navigation based on button actions in the difficulty screen.
fn difficulty_button_actions(
    query: Query<(&Interaction, &DifficultyButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game: ResMut<Game>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Clicked {
            match action {
                DifficultyButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                DifficultyButtonAction::StartGameAtDifficulty(difficulty) => {
                    *game = Game::generate(*difficulty).unwrap();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}
