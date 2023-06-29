mod board_builder;
mod board_numbers;
mod game_ui;
mod highscore_screen;
mod mode_slider;
mod wheel;

use crate::pointer_query::*;
use crate::sudoku::{self, get_x_and_y_from_pos, Game};
use crate::{ui::*, Fonts, GameTimer, Images, ScreenState, Settings};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use board_builder::{build_board, Board};
use board_numbers::*;
use game_ui::{init_game_ui, on_score_changed, on_time_changed, UiButtonAction};
use highscore_screen::{highscore_button_actions, on_highscores_changed};
use mode_slider::{slider_interaction, ModeState};
use std::num::NonZeroU8;
use std::time::Duration;
use wheel::{on_wheel_input, on_wheel_timer, render_wheel, Wheel};

pub use highscore_screen::highscore_screen_setup;
pub use wheel::SliceHandles;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selection::default())
            .init_resource::<SliceHandles>()
            .add_state::<ModeState>()
            .add_systems((
                on_keyboard_input.run_if(in_state(ScreenState::Game)),
                on_pointer_input.run_if(in_state(ScreenState::Game)),
                on_score_changed.run_if(in_state(ScreenState::Game)),
                on_highscores_changed,
                on_time_changed,
                on_timer,
            ))
            .add_systems((
                on_wheel_timer.run_if(in_state(ScreenState::Game)),
                button_actions.run_if(in_state(ScreenState::Game)),
                highscore_button_actions.run_if(in_state(ScreenState::Highscores)),
                slider_interaction.run_if(in_state(ScreenState::Game)),
                render_numbers,
                render_notes,
                render_wheel,
                render_highlights,
            ));
    }
}

#[derive(Component)]
struct Note(u8, u8, NonZeroU8);

#[derive(Component)]
struct Number(u8, u8);

#[derive(Default, Resource)]
pub struct Selection {
    pub selected_cell: Option<(u8, u8)>,
    pub hint: Option<(u8, u8)>,
    pub note_toggle: NoteToggleMode,
}

impl Selection {
    pub fn new_for_game(game: &Game) -> Self {
        let get_selected_cell = || {
            for y in 0..9 {
                for x in 0..9 {
                    if game.start.has(x, y) {
                        return Some((x, y));
                    }
                }
            }
            None
        };

        Self {
            selected_cell: get_selected_cell(),
            ..default()
        }
    }
}

#[derive(Default)]
pub enum NoteToggleMode {
    #[default]
    Set,
    Unset,
}

pub fn board_setup(
    game_screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
) {
    init_game_ui(game_screen, meshes, materials, fonts, |parent| {
        build_board(parent, fonts, game, images, settings)
    });
}

fn on_keyboard_input(
    mut game: ResMut<Game>,
    mut timer: ResMut<GameTimer>,
    mut selection: ResMut<Selection>,
    keys: Res<Input<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Up => move_selection_relative(&mut selection, 0, -1),
            Right => move_selection_relative(&mut selection, 1, 0),
            Down => move_selection_relative(&mut selection, 0, 1),
            Left => move_selection_relative(&mut selection, -1, 0),

            Key1 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 1),
            Key2 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 2),
            Key3 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 3),
            Key4 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 4),
            Key5 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 5),
            Key6 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 6),
            Key7 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 7),
            Key8 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 8),
            Key9 => handle_number_key(&mut game, &mut timer, &mut selection, &keys, 9),

            Slash => give_hint(&mut game, &mut timer, &mut selection),

            Back | Delete => clear_selection(&mut game, &selection),
            _ => {}
        }
    }
}

fn move_selection_relative(selection: &mut Selection, dx: i8, dy: i8) {
    let (x, y) = selection
        .selected_cell
        .map(|number| (number.0, number.1))
        .unwrap_or_default();

    move_selection(
        selection,
        ((x as i8 + 9 + dx) % 9) as u8,
        ((y as i8 + 9 + dy) % 9) as u8,
    );
}

fn handle_number_key(
    game: &mut Game,
    timer: &mut GameTimer,
    selection: &mut Selection,
    keys: &Input<KeyCode>,
    n: u8,
) {
    let n = NonZeroU8::new(n).unwrap();

    if keys.pressed(KeyCode::LAlt) || keys.pressed(KeyCode::RAlt) {
        toggle_note(game, selection, n);
    } else {
        fill_selected_number(game, timer, selection, n);
    }
}

fn on_pointer_input(
    mut game: ResMut<Game>,
    mut selection: ResMut<Selection>,
    timer: ResMut<GameTimer>,
    wheel: Query<&mut Wheel>,
    board: Query<&ComputedPosition, With<Board>>,
    mode: Res<State<ModeState>>,
    pointer_query: PointerQuery,
    settings: Res<Settings>,
) {
    let Some((input_kind, position)) = pointer_query.get_changed_input_with_position() else {
        return;
    };

    let Ok(board_position) = board.get_single() else {
        return;
    };

    let board_x_and_y = get_board_x_and_y(board_position, position);

    match mode.0 {
        ModeState::Normal => {
            if input_kind == InputKind::Press {
                if let Some((x, y)) = board_x_and_y {
                    move_selection(&mut selection, x, y);
                }
            }

            on_wheel_input(wheel, game, timer, input_kind, position, board, settings);
        }
        ModeState::Notes => {
            let Some((x, y)) = board_x_and_y else {
                return;
            };

            match input_kind {
                InputKind::Press => {
                    if game.current.has(x, y) {
                        move_selection(&mut selection, x, y);
                    } else if let Some(n) = selection
                        .selected_cell
                        .and_then(|(x, y)| game.current.get(x, y))
                    {
                        game.notes.toggle(x, y, n);
                        selection.note_toggle = if game.notes.has(x, y, n) {
                            NoteToggleMode::Set
                        } else {
                            NoteToggleMode::Unset
                        };
                    }
                }
                InputKind::PressedMovement => {
                    if !game.current.has(x, y) {
                        if let Some(n) = selection
                            .selected_cell
                            .and_then(|(x, y)| game.current.get(x, y))
                        {
                            match selection.note_toggle {
                                NoteToggleMode::Set => game.notes.set(x, y, n),
                                NoteToggleMode::Unset => game.notes.unset(x, y, n),
                            }
                        }
                    }
                }
                InputKind::Release => {}
            }
        }
    }
}

fn clear_selection(game: &mut Game, selection: &Selection) {
    let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) else {
        return;
    };

    if !game.start.has(x, y) {
        game.current = game.current.unset(x, y);
    }
}

fn fill_number(game: &mut Game, timer: &mut GameTimer, x: u8, y: u8, n: NonZeroU8) {
    let elapsed_secs = timer.stopwatch.elapsed_secs();
    let new_elapsed_secs = game.set(x, y, n, elapsed_secs);
    if new_elapsed_secs != elapsed_secs {
        timer
            .stopwatch
            .set_elapsed(Duration::from_secs_f32(new_elapsed_secs));
    }
}

fn fill_selected_number(
    game: &mut Game,
    timer: &mut GameTimer,
    selection: &mut Selection,
    n: NonZeroU8,
) {
    if let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) {
        fill_number(game, timer, x, y, n);

        if selection.hint == Some((x, y)) {
            selection.hint = None;
        }
    }
}

fn toggle_note(game: &mut Game, selection: &Selection, n: NonZeroU8) {
    let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) else {
        return;
    };

    game.notes.toggle(x, y, n);
}

fn move_selection(selection: &mut Selection, x: u8, y: u8) {
    let selected_cell = (x, y);
    selection.selected_cell = if selection.selected_cell == Some(selected_cell) {
        None
    } else {
        Some(selected_cell)
    };
}

fn get_board_x_and_y(board_position: &ComputedPosition, cursor_position: Vec2) -> Option<(u8, u8)> {
    let Vec2 { x, y } = cursor_position;

    if !board_position.contains(cursor_position) {
        return None;
    }

    let board_x = ((x - board_position.x) / board_position.width * 9.).floor();
    let board_y = ((y - board_position.y) / board_position.height * 9.).floor();
    Some((board_x as u8, 8 - board_y as u8))
}

fn button_actions(
    mut game: ResMut<Game>,
    mut timer: ResMut<GameTimer>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut selection: ResMut<Selection>,
    query: Query<(&Interaction, &UiButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                UiButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                UiButtonAction::Hint => give_hint(&mut game, &mut timer, &mut selection),
            }
        }
    }
}

fn give_hint(game: &mut Game, timer: &mut GameTimer, selection: &mut Selection) {
    if let Some((x, y)) = selection.hint {
        if let Some(n) = game.solution.get(x, y) {
            fill_number(game, timer, x, y, n);
            selection.hint = None;
        }
    } else if let Some(sudoku::Hint { x, y }) = game.get_hint() {
        selection.hint = Some((x, y));
    }
}

fn on_timer(
    mut game_timer: ResMut<GameTimer>,
    mut selection: ResMut<Selection>,
    screen: Res<State<ScreenState>>,
    game: Res<Game>,
    time: Res<Time>,
) {
    if game.is_solved() {
        if screen.0 == ScreenState::Game || screen.0 == ScreenState::Highscores {
            // Show a little animation for the solved state.
            let (x, y) = get_x_and_y_from_pos(((time.elapsed().as_millis() / 200) % 81) as usize);
            selection.selected_cell = Some((x, y));
        }
    } else if !game.is_default() && screen.0 == ScreenState::Game {
        game_timer.stopwatch.tick(time.delta());
    }
}
