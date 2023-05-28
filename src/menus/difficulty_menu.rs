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

    let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
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
                    *selection = get_initial_selection(&game);
                    game_timer.stopwatch.reset();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}

fn get_initial_selection(game: &Game) -> Selection {
    let get_selected_cell = || {
        for y in 0..9 {
            for x in 0..9 {
                let y = 8 - y; // Find the first in the top-left corner, instead of bottom-left.
                if game.start.has(x, y) {
                    return Some((x, y));
                }
            }
        }
        None
    };

    Selection {
        selected_cell: get_selected_cell(),
        ..default()
    }
}
