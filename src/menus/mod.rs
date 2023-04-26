mod button_builder;
mod how_to_play;
mod logo_bundle;
mod main_menu;
mod score;
mod select_difficulty;
mod settings;

use crate::{constants::*, sudoku::Game, ScreenState};
use bevy::{app::AppExit, prelude::*};
use button_builder::ButtonBuilder;
use logo_bundle::LogoBundle;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(main_menu::MainMenuPlugin)
            .add_plugin(select_difficulty::SelectDifficultyPlugin)
            .add_systems((button_actions, button_hovers));
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

// Handles changing button color based on mouse interaction.
fn button_hovers(
    mut interaction_query: Query<
        (
            &Interaction,
            &Children,
            &mut BackgroundColor,
            Option<&Secondary>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, mut bg_color, secondary) in &mut interaction_query {
        if secondary.is_some() {
            if let Some(mut text) = children
                .get(0)
                .and_then(|child| text_query.get_mut(*child).ok())
            {
                text.sections[0].style.color = match *interaction {
                    Interaction::Clicked => SECONDARY_PRESSED_BUTTON_TEXT.into(),
                    Interaction::Hovered | Interaction::None => SECONDARY_BUTTON_TEXT.into(),
                };
            }

            *bg_color = match *interaction {
                Interaction::Hovered => SECONDARY_HOVERED_BUTTON.into(),
                Interaction::Clicked | Interaction::None => SECONDARY_BUTTON.into(),
            };
        } else {
            *bg_color = match *interaction {
                Interaction::Clicked => PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_BUTTON.into(),
                Interaction::None => NORMAL_BUTTON.into(),
            };
        }
    }
}

// Updates the difficulty based on the button that was selected
fn _setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T), (Changed<Interaction>, With<Button>)>,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting) in &interaction_query {
        if *interaction == Interaction::Clicked && *setting != *button_setting {
            *setting = *button_setting;
        }
    }
}
