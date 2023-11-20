use crate::{sudoku::Difficulty, ui::*, ResourceBag, TransitionEvent};
use bevy::prelude::*;

#[derive(Component)]
pub enum DifficultyScreenButtonAction {
    BackToMain,
    StartGameAtDifficulty(Difficulty),
}

pub fn spawn_difficulty_menu_buttons(parent: &mut ChildBuilder, resources: &ResourceBag) {
    use Difficulty::*;
    use DifficultyScreenButtonAction::*;

    parent.spawn_with_children(secondary_button(
        BackToMain,
        (button_size_main(resources), button_margin_extra_height),
        text("Back", button_text(resources)),
    ));
    parent.spawn_with_children(primary_button(
        StartGameAtDifficulty(Easy),
        (button_size_main(resources), button_margin),
        text("Easy", button_text(resources)),
    ));
    parent.spawn_with_children(primary_button(
        StartGameAtDifficulty(Medium),
        (button_size_main(resources), button_margin),
        text("Medium", button_text(resources)),
    ));
    parent.spawn_with_children(primary_button(
        StartGameAtDifficulty(Advanced),
        (button_size_main(resources), button_margin),
        text("Hard", button_text(resources)),
    ));
    parent.spawn_with_children(primary_button(
        StartGameAtDifficulty(Expert),
        (button_size_main(resources), button_margin),
        text("Extreme", button_text(resources)),
    ));
}

// Handles screen navigation based on button actions in the difficulty screen.
pub fn difficulty_screen_button_actions(
    mut transition_events: EventWriter<TransitionEvent>,
    interaction_query: Query<
        (&Interaction, &DifficultyScreenButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use DifficultyScreenButtonAction::*;
            match action {
                BackToMain => transition_events.send(TransitionEvent::Exit),
                StartGameAtDifficulty(difficulty) => {
                    transition_events.send(TransitionEvent::StartGame(*difficulty))
                }
            }
        }
    }
}
