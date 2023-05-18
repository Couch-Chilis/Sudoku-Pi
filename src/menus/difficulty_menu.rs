use super::ButtonBuilder;
use crate::sudoku::Difficulty;
use crate::ui::*;
use crate::{Fonts, Game, GameTimer, ScreenState};
use bevy::prelude::*;

#[derive(Component)]
pub enum DifficultyScreenButtonAction {
    BackToMain,
    StartGameAtDifficulty(Difficulty),
}

pub fn spawn_difficulty_menu_buttons(parent: &mut ChildBuilder, fonts: &Fonts) {
    use Difficulty::*;
    use DifficultyScreenButtonAction::*;

    let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(11.));
    let buttons = ButtonBuilder::new(fonts, button_size);
    buttons.build_ternary_with_text_and_action(parent, "Back", BackToMain);
    buttons.build_with_text_and_action(parent, "Easy", StartGameAtDifficulty(Easy));
    buttons.build_with_text_and_action(parent, "Medium", StartGameAtDifficulty(Medium));
    buttons.build_with_text_and_action(parent, "Advanced", StartGameAtDifficulty(Advanced));
    buttons.build_with_text_and_action(parent, "Expert", StartGameAtDifficulty(Expert));
}

// Handles screen navigation based on button actions in the difficulty screen.
pub fn difficulty_screen_button_actions(
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    interaction_query: Query<
        (&Interaction, &DifficultyScreenButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use DifficultyScreenButtonAction::*;
            match action {
                BackToMain => screen_state.set(ScreenState::MainMenu),
                StartGameAtDifficulty(difficulty) => {
                    *game = Game::generate(*difficulty).unwrap();
                    game_timer.stopwatch.reset();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}
