mod board_builder;
mod board_line_bundle;
mod board_numbers;

use std::num::NonZeroU8;

use self::{board_builder::build_board, board_numbers::fill_numbers};
use crate::{despawn, sudoku::Game, ScreenState, WindowSize};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::default()).add_systems((
            board_setup.in_schedule(OnEnter(ScreenState::Game)),
            despawn::<OnGameScreen>.in_schedule(OnExit(ScreenState::Game)),
            keyboard_input.run_if(in_state(ScreenState::Game)),
            mouse_button_input.run_if(in_state(ScreenState::Game)),
            render_numbers.run_if(in_state(ScreenState::Game)),
            render_notes.run_if(in_state(ScreenState::Game)),
            render_selection.run_if(in_state(ScreenState::Game)),
        ));
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Note(u8, u8, NonZeroU8);

#[derive(Component)]
struct Number(u8, u8);

#[derive(Component)]
struct SelectedNumber(u8, u8);

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    build_board(&mut commands, &window_size);
    fill_numbers(&mut commands, &asset_server, &window_size);
}

fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut selected_number: Query<&mut SelectedNumber>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Up => move_selected_number_relative(&mut commands, &mut selected_number, 0, 1),
            Right => move_selected_number_relative(&mut commands, &mut selected_number, 1, 0),
            Down => move_selected_number_relative(&mut commands, &mut selected_number, 0, -1),
            Left => move_selected_number_relative(&mut commands, &mut selected_number, -1, 0),

            Key1 => handle_number_key(game.as_mut(), &keys, &selected_number, 1),
            Key2 => handle_number_key(game.as_mut(), &keys, &selected_number, 2),
            Key3 => handle_number_key(game.as_mut(), &keys, &selected_number, 3),
            Key4 => handle_number_key(game.as_mut(), &keys, &selected_number, 4),
            Key5 => handle_number_key(game.as_mut(), &keys, &selected_number, 5),
            Key6 => handle_number_key(game.as_mut(), &keys, &selected_number, 6),
            Key7 => handle_number_key(game.as_mut(), &keys, &selected_number, 7),
            Key8 => handle_number_key(game.as_mut(), &keys, &selected_number, 8),
            Key9 => handle_number_key(game.as_mut(), &keys, &selected_number, 9),

            Back | Delete => clear_selected_number(game.as_mut(), &selected_number),
            _ => {}
        }
    }
}

fn handle_number_key(
    game: &mut Game,
    keys: &Input<KeyCode>,
    selected_number: &Query<&mut SelectedNumber>,
    n: u8,
) {
    let n = NonZeroU8::new(n).unwrap();

    if keys.pressed(KeyCode::LAlt) || keys.pressed(KeyCode::RAlt) {
        toggle_note(game, selected_number, n);
    } else {
        fill_selected_number(game, selected_number, n);
    }
}

fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut selected_number: Query<&mut SelectedNumber>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    window_size: Res<WindowSize>,
) {
    let Some(cursor_position) = window_query.get_single().ok().and_then(|window| window.cursor_position()) else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        if let Some((x, y)) = get_board_x_and_y(&window_size, cursor_position) {
            move_selected_number(&mut commands, &mut selected_number, x, y);
        }
    }
}

fn render_numbers(game: Res<Game>, mut number_query: Query<(&Number, &mut Text)>) {
    if !game.is_changed() {
        return;
    }

    for (Number(x, y), mut text) in &mut number_query {
        if let Some(n) = game.current.get(*x, *y) {
            text.sections[0].value = n.to_string();

            text.sections[0].style.color = if game.start.has(*x, *y) {
                Color::BLACK
            } else {
                Color::BLUE
            }
        } else {
            text.sections[0].style.color = Color::NONE;
        };
    }
}

fn render_notes(game: Res<Game>, mut note_query: Query<(&Note, &mut Text)>) {
    if !game.is_changed() {
        return;
    }

    for (Note(x, y, n), mut text) in &mut note_query {
        text.sections[0].style.color = if game.notes.has(*x, *y, *n) && !game.current.has(*x, *y) {
            Color::BLACK
        } else {
            Color::NONE
        };
    }
}

fn render_selection(
    mut selected_number_query: Query<(&SelectedNumber, &mut Transform), Changed<SelectedNumber>>,
    window_size: Res<WindowSize>,
) {
    for (selected_number, mut transform) in &mut selected_number_query {
        let scale = 10. * window_size.vmin_scale;
        let SelectedNumber(x, y) = selected_number;
        transform.scale = Vec3::new(scale, scale, 0.);
        transform.translation = Vec3::new((*x as f32 - 4.) * scale, (*y as f32 - 4.) * scale, 1.);
    }
}

fn clear_selected_number(game: &mut Game, selected_number: &Query<&mut SelectedNumber>) {
    let Ok((x, y)) = selected_number.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    if !game.start.has(x, y) {
        game.current = game.current.unset(x, y);
    }
}

fn fill_selected_number(
    game: &mut Game,
    selected_number: &Query<&mut SelectedNumber>,
    n: NonZeroU8,
) {
    let Ok((x, y)) = selected_number.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    if game.start.has(x, y) {
        return; // Starting numbers are fixed and may not be replaced.
    }

    game.current = game.current.set(x, y, n);
    game.notes.remove_all_notes_affected_by_set(x, y, n);
}

fn toggle_note(game: &mut Game, selected_number: &Query<&mut SelectedNumber>, n: NonZeroU8) {
    let Ok((x, y)) = selected_number.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    game.notes.toggle(x, y, n);
}

fn move_selected_number(
    commands: &mut Commands,
    selected_number: &mut Query<&mut SelectedNumber>,
    x: u8,
    y: u8,
) {
    if let Ok(mut selected_number) = selected_number.get_single_mut() {
        selected_number.0 = x;
        selected_number.1 = y;
    } else {
        commands.spawn((
            SelectedNumber(x, y),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.8, 0.7, 0.0, 0.5),
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ));
    }
}

fn move_selected_number_relative(
    commands: &mut Commands,
    selected_number: &mut Query<&mut SelectedNumber>,
    dx: i8,
    dy: i8,
) {
    let (x, y) = selected_number
        .get_single()
        .map(|number| (number.0, number.1))
        .unwrap_or_default();

    move_selected_number(
        commands,
        selected_number,
        ((x as i8 + 9 + dx) % 9) as u8,
        ((y as i8 + 9 + dy) % 9) as u8,
    );
}

fn get_board_x_and_y(window_size: &WindowSize, cursor_position: Vec2) -> Option<(u8, u8)> {
    let Vec2 { x, y } = cursor_position;

    let board_size = 90. * window_size.vmin_scale;
    let board_offset_x = 0.5 * (window_size.width - board_size);
    let board_offset_y = 0.5 * (window_size.height - board_size);

    if x < board_offset_x
        || x > window_size.width - board_offset_x
        || y < board_offset_y
        || y > window_size.height - board_offset_y
    {
        return None;
    }

    let board_x = ((x - board_offset_x) / board_size * 9.).floor();
    let board_y = ((y - board_offset_y) / board_size * 9.).floor();
    Some((board_x as u8, board_y as u8))
}
