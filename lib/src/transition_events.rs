use crate::{game::*, sudoku::*, GameTimer, ScreenState, Settings};
use bevy::prelude::*;
use std::num::NonZeroU8;

#[derive(Clone, Copy, Event)]
pub enum TransitionEvent {
    ContinueGame,
    FinishOnboarding,
    HowToPlayNotes,
    HowToPlayNumbers,
    StartGame(Difficulty),
}

pub fn on_transition(
    mut reader: EventReader<TransitionEvent>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut mode_state: ResMut<NextState<ModeState>>,
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut selection: ResMut<Selection>,
    mut settings: ResMut<Settings>,
) {
    for event in &mut reader {
        use TransitionEvent::*;
        match event {
            ContinueGame => {
                *selection = Selection::new_for_game(&game);
                mode_state.set(ModeState::Normal);
                screen_state.set(ScreenState::Game);
            }
            FinishOnboarding => {
                // Had the onboarding been finished before?
                if settings.onboarding_finished {
                    *game = Game::load();

                    screen_state.set(ScreenState::MainMenu);
                } else {
                    settings.onboarding_finished = true;
                    settings.save();

                    *game = Game::default();

                    screen_state.set(ScreenState::SelectDifficulty);
                }
            }
            HowToPlayNotes => {
                *game = Game::load_tutorial();
                game.set(
                    6,
                    4,
                    NonZeroU8::new(2).unwrap(),
                    SetNumberOptions::default(),
                );
                *selection = Selection {
                    selected_cell: Some((6, 4)),
                    selected_note: None,
                    hint: Some((3, 2)),
                    note_toggle: None,
                };
                mode_state.set(ModeState::Notes);
                screen_state.set(ScreenState::HowToPlayNotes);
            }
            HowToPlayNumbers => {
                *game = Game::load_tutorial();
                *selection = Selection {
                    selected_cell: None,
                    selected_note: None,
                    hint: Some((6, 4)),
                    note_toggle: None,
                };
                mode_state.set(ModeState::Normal);
                screen_state.set(ScreenState::HowToPlayNumbers);
            }
            StartGame(difficulty) => {
                *game = Game::generate(*difficulty).expect("Could not generate game");
                *selection = Selection::new_for_game(&game);
                mode_state.set(ModeState::Normal);
                screen_state.set(ScreenState::Game);
                game_timer.elapsed_secs = 0.;
            }
        }
    }
}
