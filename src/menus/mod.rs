mod button_builder;
mod how_to_play;
mod logo_builder;
mod main_menu;
mod score;
mod select_difficulty;
mod settings;

use crate::sudoku::Game;
use crate::ui::{Button, Interaction};
use crate::ScreenState;
use bevy::{app::AppExit, prelude::*};
use button_builder::ButtonBuilder;
use logo_builder::build_logo;

pub use main_menu::main_menu_setup;
//pub use select_difficulty::select_difficulty_setup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_actions);
    }
}

#[derive(Component)]
pub enum MenuButtonAction {
    BackToMain,
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    //GoToOptions,
    StartGameAtDifficulty(u8),
    Quit,
}

#[derive(Component)]
pub struct Secondary;

// Handles screen navigation based on button actions.
fn button_actions(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game: ResMut<Game>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Clicked {
            match action {
                MenuButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                MenuButtonAction::ContinueGame => screen_state.set(ScreenState::Game),
                MenuButtonAction::GoToHowToPlay => screen_state.set(ScreenState::HowToPlay),
                MenuButtonAction::GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                //MenuButtonAction::GoToOptions => screen_state.set(ScreenState::Options),
                MenuButtonAction::StartGameAtDifficulty(difficulty) => {
                    *game = Game::generate(*difficulty).unwrap();
                    screen_state.set(ScreenState::Game);
                }
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
