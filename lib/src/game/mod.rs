mod board_builder;
mod board_numbers;
mod game_ui;
mod highscore_screen;
mod mode_slider;
mod wheel;

use crate::pointer_query::*;
use crate::sudoku::{self, get_x_and_y_from_pos, Game, Notes, SetNumberOptions};
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
use wheel::{on_wheel_input, on_wheel_timer, render_wheel};

pub use highscore_screen::highscore_screen_setup;
pub use wheel::{SliceHandles, Wheel};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selection::default())
            .init_resource::<SliceHandles>()
            .add_state::<ModeState>()
            .add_systems(
                Update,
                (
                    on_keyboard_input.run_if(in_state(ScreenState::Game)),
                    on_pointer_input.run_if(in_state(ScreenState::Game)),
                    on_score_changed.run_if(in_state(ScreenState::Game)),
                    on_highscores_changed,
                    on_time_changed,
                    on_timer,
                ),
            )
            .add_systems(
                Update,
                (
                    on_wheel_timer.run_if(in_state(ScreenState::Game)),
                    button_actions.run_if(in_state(ScreenState::Game)),
                    highscore_button_actions.run_if(in_state(ScreenState::Highscores)),
                    slider_interaction.run_if(in_state(ScreenState::Game)),
                    render_numbers,
                    render_notes,
                    render_wheel,
                    render_highlights,
                ),
            );
    }
}

#[derive(Component)]
pub struct Note {
    x: u8,
    y: u8,
    n: NonZeroU8,
    animation_kind: Option<NoteAnimationKind>,
    animation_timer: f32,
}

impl Note {
    fn new(x: u8, y: u8, n: NonZeroU8) -> Self {
        Self {
            x,
            y,
            n,
            animation_kind: None,
            animation_timer: 0.,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum NoteAnimationKind {
    Mistake,
    FadeOut(Duration),
}

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
    mut notes: Query<&mut Note>,
    settings: Res<Settings>,
    keys: Res<Input<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Up => move_selection_relative(&mut selection, 0, -1),
            Right => move_selection_relative(&mut selection, 1, 0),
            Down => move_selection_relative(&mut selection, 0, 1),
            Left => move_selection_relative(&mut selection, -1, 0),

            Key1 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                1,
            ),
            Key2 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                2,
            ),
            Key3 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                3,
            ),
            Key4 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                4,
            ),
            Key5 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                5,
            ),
            Key6 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                6,
            ),
            Key7 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                7,
            ),
            Key8 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                8,
            ),
            Key9 => handle_number_key(
                &mut game,
                &mut timer,
                &mut selection,
                &mut notes,
                &settings,
                &keys,
                9,
            ),

            Slash => give_hint(&mut game, &mut timer, &mut selection, &mut notes, &settings),

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
    notes: &mut Query<&mut Note>,
    settings: &Settings,
    keys: &Input<KeyCode>,
    n: u8,
) {
    let n = NonZeroU8::new(n).unwrap();

    if keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight) {
        toggle_note(game, selection, n);
    } else {
        fill_selected_number(game, timer, selection, notes, settings, n);
    }
}

fn on_pointer_input(
    mut game: ResMut<Game>,
    mut selection: ResMut<Selection>,
    notes: Query<&mut Note>,
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

    match mode.get() {
        ModeState::Normal => {
            if input_kind == InputKind::Press {
                if let Some((x, y)) = board_x_and_y {
                    move_selection(&mut selection, x, y);
                }
            }

            on_wheel_input(
                wheel, game, selection, notes, timer, input_kind, position, board, settings,
            );
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

fn fill_number(
    game: &mut Game,
    timer: &mut GameTimer,
    selection: &mut Selection,
    notes: &mut Query<&mut Note>,
    settings: &Settings,
    is_hint: bool,
    x: u8,
    y: u8,
    n: NonZeroU8,
) {
    let elapsed_secs = timer.stopwatch.elapsed_secs();
    let options = SetNumberOptions {
        elapsed_secs,
        is_hint,
        show_mistakes: settings.show_mistakes,
    };

    let previous_notes = game.notes.clone();

    let new_elapsed_secs = game.set(x, y, n, options);
    if new_elapsed_secs != elapsed_secs {
        animate_mistake(notes, x, y, n);

        timer
            .stopwatch
            .set_elapsed(Duration::from_secs_f32(new_elapsed_secs));
    }

    if selection.hint == Some((x, y)) {
        selection.hint = None;
    }

    animate_cleared_notes(notes, &game.notes, &previous_notes, x, y);
}

fn animate_cleared_notes(
    notes: &mut Query<&mut Note>,
    current_notes: &Notes,
    previous_notes: &Notes,
    set_x: u8,
    set_y: u8,
) {
    let cleared_notes = current_notes.get_cleared_since(previous_notes);
    for mut note in notes {
        let x = note.x;
        let y = note.y;
        let n = note.n;

        if (x != set_x || y != set_y) && cleared_notes.contains(&(x, y, n)) {
            let distance = (((x as f32) - (set_x as f32)).powi(2)
                + ((y as f32) - (set_y as f32)).powi(2))
            .sqrt();
            note.animation_kind = Some(NoteAnimationKind::FadeOut(Duration::from_secs_f32(
                0.05 * distance,
            )));
            note.animation_timer = 0.;
        }
    }
}

fn animate_mistake(notes: &mut Query<&mut Note>, set_x: u8, set_y: u8, set_n: NonZeroU8) {
    for mut note in notes {
        if note.x == set_x && note.y == set_y && note.n == set_n {
            note.animation_kind = Some(NoteAnimationKind::Mistake);
            note.animation_timer = 0.;
        }
    }
}

fn fill_selected_number(
    game: &mut Game,
    timer: &mut GameTimer,
    selection: &mut Selection,
    notes: &mut Query<&mut Note>,
    settings: &Settings,
    n: NonZeroU8,
) {
    if let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) {
        fill_number(game, timer, selection, notes, settings, false, x, y, n);
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
    mut notes: Query<&mut Note>,
    settings: Res<Settings>,
    query: Query<(&Interaction, &UiButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                UiButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                UiButtonAction::Hint => {
                    give_hint(&mut game, &mut timer, &mut selection, &mut notes, &settings)
                }
            }
        }
    }
}

fn give_hint(
    game: &mut Game,
    timer: &mut GameTimer,
    selection: &mut Selection,
    notes: &mut Query<&mut Note>,
    settings: &Settings,
) {
    if let Some((x, y)) = selection.hint {
        if let Some(n) = game.solution.get(x, y) {
            fill_number(game, timer, selection, notes, settings, true, x, y, n);
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
        if screen.get() == &ScreenState::Game || screen.get() == &ScreenState::Highscores {
            // Show a little animation for the solved state.
            let (x, y) = get_x_and_y_from_pos(((time.elapsed().as_millis() / 200) % 81) as usize);
            selection.selected_cell = Some((x, y));
        }
    } else if !game.is_default() && screen.get() == &ScreenState::Game {
        game_timer.stopwatch.tick(time.delta());
    }
}
