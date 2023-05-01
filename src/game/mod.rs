mod board_builder;
mod board_numbers;
mod game_ui;

use crate::constants::{CELL_SCALE, CELL_SIZE};
use crate::sudoku::{self, get_pos, get_x_and_y_from_pos, Game};
use crate::ui::{Button, ComputedPosition, Interaction};
use crate::{Fonts, ScreenState};
use bevy::ecs::system::EntityCommands;
use bevy::{prelude::*, window::PrimaryWindow};
use board_builder::{build_board, Board};
use game_ui::{init_game_ui, UiButtonAction};
use std::num::NonZeroU8;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::load())
            .insert_resource(Selection::default())
            .add_systems((
                keyboard_input.run_if(in_state(ScreenState::Game)),
                mouse_button_input.run_if(in_state(ScreenState::Game)),
                render_numbers.run_if(in_state(ScreenState::Game)),
                render_notes.run_if(in_state(ScreenState::Game)),
                render_highlights.run_if(in_state(ScreenState::Game)),
                button_actions.run_if(in_state(ScreenState::Game)),
            ));
    }
}

#[derive(Component)]
struct Note(u8, u8, NonZeroU8);

#[derive(Component)]
struct Number(u8, u8);

#[derive(Default, Resource)]
struct Selection {
    selected_cell: Option<(u8, u8)>,
    hint: Option<(u8, u8)>,
}

#[derive(Component)]
struct HighlightedNumber(usize, HighlightKind);

#[derive(Clone, Copy)]
enum HighlightKind {
    Selection,
    SameNumber,
    InRange,
    Hint,
}

pub fn board_setup(game_screen: &mut EntityCommands, fonts: &Fonts, game: &Game) {
    init_game_ui(game_screen, fonts, |parent| {
        build_board(parent, fonts, game)
    });
}

fn keyboard_input(
    mut game: ResMut<Game>,
    mut selection: ResMut<Selection>,
    keys: Res<Input<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Up => move_selection_relative(&mut selection, 0, 1),
            Right => move_selection_relative(&mut selection, 1, 0),
            Down => move_selection_relative(&mut selection, 0, -1),
            Left => move_selection_relative(&mut selection, -1, 0),

            Key1 => handle_number_key(&mut game, &keys, &selection, 1),
            Key2 => handle_number_key(&mut game, &keys, &selection, 2),
            Key3 => handle_number_key(&mut game, &keys, &selection, 3),
            Key4 => handle_number_key(&mut game, &keys, &selection, 4),
            Key5 => handle_number_key(&mut game, &keys, &selection, 5),
            Key6 => handle_number_key(&mut game, &keys, &selection, 6),
            Key7 => handle_number_key(&mut game, &keys, &selection, 7),
            Key8 => handle_number_key(&mut game, &keys, &selection, 8),
            Key9 => handle_number_key(&mut game, &keys, &selection, 9),

            Slash => give_hint(&mut game, &mut selection),

            Back | Delete => clear_selection(&mut game, &mut selection),
            _ => {}
        }
    }
}

fn handle_number_key(game: &mut Game, keys: &Input<KeyCode>, selection: &Selection, n: u8) {
    let n = NonZeroU8::new(n).unwrap();

    if keys.pressed(KeyCode::LAlt) || keys.pressed(KeyCode::RAlt) {
        toggle_note(game, selection, n);
    } else {
        fill_selected_number(game, selection, n);
    }
}

fn mouse_button_input(
    mut selection: ResMut<Selection>,
    buttons: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    board: Query<&ComputedPosition, With<Board>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let Some(cursor_position) = primary_window.get_single().ok().and_then(|window| window.cursor_position()) else {
            return;
        };

        let Ok(board_position) = board.get_single() else {
            return;
        };

        if let Some((x, y)) = get_board_x_and_y(board_position, cursor_position) {
            move_selection(&mut selection, x, y);
        }
    }
}

fn render_numbers(mut numbers: Query<(&Number, &mut Text)>, game: Res<Game>) {
    if !game.is_changed() {
        return;
    }

    for (Number(x, y), mut text) in &mut numbers {
        if let Some(n) = game.current.get(*x, *y) {
            text.sections[0].value = n.to_string();
            text.sections[0].style.color = get_number_color(&game, *x, *y);
        } else {
            text.sections[0].style.color = Color::NONE;
        };
    }
}

fn get_number_color(game: &Game, x: u8, y: u8) -> Color {
    if game.start.has(x, y) {
        Color::BLACK
    } else {
        Color::BLUE
    }
}

fn render_notes(mut notes: Query<(&Note, &mut Text)>, game: Res<Game>) {
    if !game.is_changed() {
        return;
    }

    for (Note(x, y, n), mut text) in &mut notes {
        text.sections[0].style.color = if game.notes.has(*x, *y, *n) && !game.current.has(*x, *y) {
            Color::BLACK
        } else {
            Color::NONE
        };
    }
}

fn render_highlights(
    mut commands: Commands,
    game: Res<Game>,
    selection: Res<Selection>,
    board: Query<Entity, With<Board>>,
    highlighted_numbers: Query<Entity, With<HighlightedNumber>>,
) {
    if !game.is_changed() && !selection.is_changed() {
        return;
    }

    for entity in &highlighted_numbers {
        commands.entity(entity).despawn();
    }

    let Ok(mut board) = board.get_single().map(|board| commands.entity(board)) else {
        return;
    };

    // First determine the type of highlight each cell should receive:
    let mut highlights = [None; 81];
    if let Some((x, y)) = selection.selected_cell {
        let selected_pos = get_pos(x, y);

        let selected_cell = game.current.get(x, y);
        if selected_cell.is_some() {
            // Find all the cells within range.
            for pos in 0..81 {
                if game.current.get_by_pos(pos) == selected_cell {
                    let (x, y) = get_x_and_y_from_pos(pos);
                    for i in 0..9 {
                        highlights[get_pos(x, i)] = Some(HighlightKind::InRange);
                        highlights[get_pos(i, y)] = Some(HighlightKind::InRange);
                    }
                }
            }

            // Find all the cells with the same number.
            for pos in 0..81 {
                if pos != selected_pos && game.current.get_by_pos(pos) == selected_cell {
                    highlights[pos] = Some(HighlightKind::SameNumber);
                }
            }
        }

        highlights[selected_pos] = Some(HighlightKind::Selection);
    }
    if let Some((x, y)) = selection.hint {
        highlights[get_pos(x, y)] = Some(HighlightKind::Hint);
    }

    board.with_children(|parent| {
        for (pos, highlight) in highlights.into_iter().enumerate() {
            if let Some(highlight_kind) = highlight {
                let (x, y) = get_x_and_y_from_pos(pos);
                let color = match highlight_kind {
                    HighlightKind::Selection => Color::rgba(0.9, 0.8, 0.0, 0.7),
                    HighlightKind::SameNumber => Color::rgba(0.9, 0.8, 0.0, 0.45),
                    HighlightKind::InRange => Color::rgba(0.9, 0.8, 0.0, 0.2),
                    HighlightKind::Hint => Color::rgba(0.2, 0.9, 0.0, 0.35),
                };

                parent.spawn((
                    HighlightedNumber(pos, highlight_kind),
                    SpriteBundle {
                        sprite: Sprite { color, ..default() },
                        transform: get_cell_transform(x, y).with_scale(CELL_SCALE),
                        ..default()
                    },
                ));
            }
        }
    });
}

fn get_cell_transform(x: u8, y: u8) -> Transform {
    Transform::from_translation(Vec3::new(
        (x as f32 - 4.) * CELL_SIZE,
        (y as f32 - 4.) * CELL_SIZE,
        1.,
    ))
}

fn clear_selection(game: &mut Game, selection: &Selection) {
    let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) else {
        return;
    };

    if !game.start.has(x, y) {
        game.current = game.current.unset(x, y);
    }
}

fn fill_selected_number(game: &mut Game, selection: &Selection, n: NonZeroU8) {
    if let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) {
        game.set(x, y, n);
    }
}

fn toggle_note(game: &mut Game, selection: &Selection, n: NonZeroU8) {
    let Some((x, y)) = selection.selected_cell.map(|number| (number.0, number.1)) else {
        return;
    };

    game.notes.toggle(x, y, n);
}

fn move_selection(selection: &mut Selection, x: u8, y: u8) {
    selection.selected_cell = Some((x, y));
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

fn get_board_x_and_y(board_position: &ComputedPosition, cursor_position: Vec2) -> Option<(u8, u8)> {
    let Vec2 { x, y } = cursor_position;

    if !board_position.contains(cursor_position) {
        return None;
    }

    let board_x = ((x - board_position.x) / board_position.width * 9.).floor();
    let board_y = ((y - board_position.y) / board_position.height * 9.).floor();
    Some((board_x as u8, board_y as u8))
}

fn button_actions(
    mut game: ResMut<Game>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut selection: ResMut<Selection>,
    query: Query<(&Interaction, &UiButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Clicked {
            match action {
                UiButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                UiButtonAction::Hint => give_hint(&mut game, &mut selection),
                _ => {}
            }
        }
    }
}

fn give_hint(game: &mut Game, selection: &mut Selection) {
    if let Some((x, y)) = selection.hint {
        if let Some(n) = game.solution.get(x, y) {
            game.set(x, y, n);
            selection.hint = None;
        }
    } else if let Some(sudoku::Hint { x, y }) = game.current.get_hint() {
        selection.hint = Some((x, y));
    }
}
