use crate::{game::*, sudoku::*, GameTimer, ScreenState, Settings};
use bevy::app::AppExit;
use bevy::prelude::*;
use std::num::NonZeroU8;

#[derive(Clone, Copy, Event)]
pub enum TransitionEvent {
    ContinueGame,
    Exit,
    FinishOnboarding,
    LearnNotes,
    LearnNumbers,
    StartGame(Difficulty),
}

pub fn on_transition(
    current_state: Res<State<ScreenState>>,
    mut reader: EventReader<TransitionEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut mode_state: ResMut<NextState<ModeState>>,
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut selection: ResMut<Selection>,
    mut settings: ResMut<Settings>,
) {
    for event in reader.read() {
        use TransitionEvent::*;
        match event {
            ContinueGame => {
                *selection = Selection::new_for_game(&game);
                mode_state.set(ModeState::Normal);
                screen_state.set(ScreenState::Highscores);
            }
            Exit => match current_state.get() {
                ScreenState::MainMenu => app_exit_events.send(AppExit),
                ScreenState::Settings => screen_state.set(ScreenState::Game),
                ScreenState::LearnNumbers => how_to_play_notes(
                    &mut screen_state,
                    &mut mode_state,
                    &mut game,
                    &mut selection,
                ),
                ScreenState::LearnNotes => {
                    finish_onboarding(&mut screen_state, &mut game, &mut settings)
                }
                _ => screen_state.set(ScreenState::MainMenu),
            },
            FinishOnboarding => finish_onboarding(&mut screen_state, &mut game, &mut settings),
            LearnNotes => how_to_play_notes(
                &mut screen_state,
                &mut mode_state,
                &mut game,
                &mut selection,
            ),
            LearnNumbers => how_to_play_numbers(
                &mut screen_state,
                &mut mode_state,
                &mut game,
                &mut selection,
            ),
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

fn finish_onboarding(
    screen_state: &mut ResMut<NextState<ScreenState>>,
    game: &mut ResMut<Game>,
    settings: &mut ResMut<Settings>,
) {
    // Had the onboarding been finished before?
    if settings.onboarding_finished {
        **game = Game::load();

        screen_state.set(ScreenState::MainMenu);
    } else {
        settings.onboarding_finished = true;
        settings.save();

        **game = Game::default();

        screen_state.set(ScreenState::SelectDifficulty);
    }
}

fn how_to_play_notes(
    screen_state: &mut ResMut<NextState<ScreenState>>,
    mode_state: &mut ResMut<NextState<ModeState>>,
    game: &mut ResMut<Game>,
    selection: &mut ResMut<Selection>,
) {
    **game = Game::load_tutorial();
    game.set(
        6,
        4,
        NonZeroU8::new(2).unwrap(),
        SetNumberOptions::default(),
    );
    **selection = Selection {
        selected_cell: Some((6, 4)),
        selected_note: None,
        hint: Some((3, 2)),
        note_toggle: None,
    };
    mode_state.set(ModeState::Notes);
    screen_state.set(ScreenState::LearnNotes);
}

fn how_to_play_numbers(
    screen_state: &mut ResMut<NextState<ScreenState>>,
    mode_state: &mut ResMut<NextState<ModeState>>,
    game: &mut ResMut<Game>,
    selection: &mut ResMut<Selection>,
) {
    **game = Game::load_tutorial();
    **selection = Selection {
        selected_cell: None,
        selected_note: None,
        hint: Some((6, 4)),
        note_toggle: None,
    };
    mode_state.set(ModeState::Normal);
    screen_state.set(ScreenState::LearnNumbers);
}
