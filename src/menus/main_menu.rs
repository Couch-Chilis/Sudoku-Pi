use super::ButtonBuilder;
use crate::{sudoku::*, ui::*};
use crate::{Fonts, ScreenState};
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub enum MainScreenButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    Quit,
}

pub fn spawn_main_menu_buttons(main_section: &mut ChildBuilder, fonts: &Fonts, game: &Game) {
    use MainScreenButtonAction::*;
    let buttons = ButtonBuilder::new(fonts);
    buttons.add_ternary_with_text_and_action(main_section, "Quit", Quit);
    buttons.add_secondary_with_text_and_action(main_section, "How to Play", GoToHowToPlay);
    if game.may_continue() {
        buttons.add_secondary_with_text_and_action(main_section, "New Game", GoToNewGame);
        buttons.add_with_text_and_action(main_section, "Continue", ContinueGame);
    } else {
        buttons.add_with_text_and_action(main_section, "New Game", GoToNewGame);
    }
}

pub fn main_menu_button_actions(
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
    interaction_query: Query<(&Interaction, &MainScreenButtonAction), Changed<Interaction>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::JustPressed {
            use MainScreenButtonAction::*;
            match action {
                ContinueGame => screen_state.set(ScreenState::Game),
                GoToHowToPlay => screen_state.set(ScreenState::Highscores),
                GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
