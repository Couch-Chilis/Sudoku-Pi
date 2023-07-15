use super::ButtonBuilder;
use crate::{game::Selection, sudoku::Difficulty, ui::*};
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

    let button_style = FlexItemStyle::fixed_size(Val::Vmin(70.), Val::Vmin(10.))
        .with_margin(Size::all(Val::Vmin(1.5)));
    let buttons = ButtonBuilder::new(fonts, button_style);
    buttons.build_secondary_with_text_and_action_and_custom_margin(
        parent,
        "Back",
        BackToMain,
        Size::new(Val::Vmin(1.5), Val::Vmin(5.)),
    );
    buttons.build_with_text_and_action(parent, "Easy", StartGameAtDifficulty(Easy));
    buttons.build_selected_with_text_and_action(parent, "Medium", StartGameAtDifficulty(Medium));
    buttons.build_with_text_and_action(parent, "Hard", StartGameAtDifficulty(Advanced));
    buttons.build_with_text_and_action(parent, "Extreme", StartGameAtDifficulty(Expert));
}

// Handles screen navigation based on button actions in the difficulty screen.
pub fn difficulty_screen_button_actions(
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut selection: ResMut<Selection>,
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
                    *selection = Selection::new_for_game(&game);
                    game_timer.stopwatch.reset();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}
