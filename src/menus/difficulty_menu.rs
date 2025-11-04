use crate::{sudoku::Difficulty, ui::*, Transition};
use bevy::prelude::*;

#[derive(Component)]
pub enum DifficultyScreenButtonAction {
    BackToMain,
    StartGameAtDifficulty(Difficulty),
}

pub fn difficulty_menu_buttons() -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    use Difficulty::*;
    use DifficultyScreenButtonAction::*;

    fragment5(
        secondary_button(
            BackToMain,
            (button_size_main, button_margin_extra_height),
            text("Back", button_text),
        ),
        primary_button(
            StartGameAtDifficulty(Easy),
            (button_size_main, button_margin),
            text("Easy", button_text),
        ),
        primary_button(
            StartGameAtDifficulty(Medium),
            (button_size_main, button_margin),
            text("Medium", button_text),
        ),
        primary_button(
            StartGameAtDifficulty(Advanced),
            (button_size_main, button_margin),
            text("Hard", button_text),
        ),
        primary_button(
            StartGameAtDifficulty(Expert),
            (button_size_main, button_margin),
            text("Extreme", button_text),
        ),
    )
}

// Handles screen navigation based on button actions in the difficulty screen.
pub fn difficulty_screen_button_actions(
    mut transitions: MessageWriter<Transition>,
    interaction_query: Query<
        (&Interaction, &DifficultyScreenButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use DifficultyScreenButtonAction::*;
            match action {
                BackToMain => {
                    transitions.write(Transition::Exit);
                }
                StartGameAtDifficulty(difficulty) => {
                    transitions.write(Transition::StartGame(*difficulty));
                }
            }
        }
    }
}
