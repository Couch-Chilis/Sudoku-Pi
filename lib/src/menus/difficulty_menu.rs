use super::ButtonBuilder;
use crate::{sudoku::Difficulty, ui::*, ScreenState, TransitionEvent};
use bevy::prelude::*;

#[derive(Component)]
pub enum DifficultyScreenButtonAction {
    BackToMain,
    StartGameAtDifficulty(Difficulty),
}

pub fn spawn_difficulty_menu_buttons(parent: &mut ChildBuilder, buttons: &ButtonBuilder) {
    use Difficulty::*;
    use DifficultyScreenButtonAction::*;

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
    mut screen_state: ResMut<NextState<ScreenState>>,
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
                BackToMain => screen_state.set(ScreenState::MainMenu),
                StartGameAtDifficulty(difficulty) => {
                    transition_events.send(TransitionEvent::StartGame(*difficulty))
                }
            }
        }
    }
}
