mod board_builder;
mod board_line_bundle;
mod board_numbers;
mod ui;

use crate::despawn;
use crate::sudoku::{self, get_pos, get_x_and_y_from_pos, Game};
use crate::{ScreenState, WindowSize};
use bevy::{prelude::*, window::PrimaryWindow};
use board_builder::build_board;
use board_numbers::fill_numbers;
use std::num::NonZeroU8;
use ui::{init_ui, UiButtonAction};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::load()).add_systems((
            board_setup.in_schedule(OnEnter(ScreenState::Game)),
            render_numbers
                .in_schedule(OnEnter(ScreenState::Game))
                .after(board_setup),
            despawn::<OnGameScreen>.in_schedule(OnExit(ScreenState::Game)),
            keyboard_input.run_if(in_state(ScreenState::Game)),
            mouse_button_input.run_if(in_state(ScreenState::Game)),
            render_changed_numbers.run_if(in_state(ScreenState::Game)),
            render_notes.run_if(in_state(ScreenState::Game)),
            render_hint.run_if(in_state(ScreenState::Game)),
            render_selection.run_if(in_state(ScreenState::Game)),
            update_highlights.run_if(in_state(ScreenState::Game)),
            remove_hint.run_if(in_state(ScreenState::Game)),
            button_actions.run_if(in_state(ScreenState::Game)),
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

#[derive(Component)]
struct HighlightedNumber(usize);

#[derive(Component)]
struct Hint(u8, u8);

type SelectionQuery<'world, 'state, 'selection> =
    Query<'world, 'state, &'selection mut SelectedNumber>;

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    build_board(&mut commands, &window_size);
    fill_numbers(&mut commands, &asset_server, &window_size);
    init_ui(&asset_server, &mut commands, &window_size);
}

fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    hint: Query<&Hint>,
    mut selected_number: SelectionQuery,
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

            Slash => give_hint(&mut commands, game.as_mut(), &hint),

            Back | Delete => clear_selected_number(game.as_mut(), &selected_number),
            _ => {}
        }
    }
}

fn handle_number_key(
    game: &mut Game,
    keys: &Input<KeyCode>,
    selected_number: &SelectionQuery,
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
    mut selected_number: SelectionQuery,
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

fn render_changed_numbers(game: Res<Game>, number_query: Query<(&Number, &mut Text)>) {
    if game.is_changed() {
        render_numbers(game, number_query);
    }
}

fn render_numbers(game: Res<Game>, mut number_query: Query<(&Number, &mut Text)>) {
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
    for (SelectedNumber(x, y), mut transform) in &mut selected_number_query {
        let scale = 10. * window_size.vmin_scale;
        transform.scale = Vec3::new(scale, scale, 0.);
        transform.translation = translate_to_cell_size(*x, *y, scale);
    }
}

fn render_hint(
    mut hint_query: Query<(&Hint, &mut Transform), Changed<Hint>>,
    window_size: Res<WindowSize>,
) {
    for (Hint(x, y), mut transform) in &mut hint_query {
        let scale = 10. * window_size.vmin_scale;
        transform.scale = Vec3::new(scale, scale, 0.);
        transform.translation = translate_to_cell_size(*x, *y, scale);
    }
}

fn remove_hint(
    commands: Commands,
    hint_query: Query<&Hint>,
    hint_entity: Query<Entity, With<Hint>>,
    game: Res<Game>,
) {
    if !game.is_changed() {
        return;
    }

    let Ok((x, y)) = hint_query.get_single().map(|hint| (hint.0, hint.1)) else {
        return;
    };

    if game.current.has(x, y) {
        despawn(hint_entity, commands)
    }
}

fn update_highlights(
    mut commands: Commands,
    selected_number_query: Query<&SelectedNumber>,
    selected_number_changed_query: Query<&SelectedNumber, Changed<SelectedNumber>>,
    highlighted_number_query: Query<Entity, With<HighlightedNumber>>,
    game: Res<Game>,
    window_size: Res<WindowSize>,
) {
    if !game.is_changed() && !selected_number_changed_query.get_single().is_ok() {
        return;
    }

    let Ok((x, y)) = selected_number_query.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    for entity in &highlighted_number_query {
        commands.entity(entity).despawn();
    }

    let selected_cell = game.current.get(x, y);
    if selected_cell.is_none() {
        return;
    };

    let current_pos = get_pos(x, y);
    let scale = 10. * window_size.vmin_scale;

    commands.spawn_batch(
        (0..81)
            .filter(|pos| *pos != current_pos)
            .filter(|pos| game.current.get_by_pos(*pos) == selected_cell)
            .map(|pos| {
                let (x, y) = get_x_and_y_from_pos(pos);
                (
                    HighlightedNumber(pos),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.7, 0.0, 0.35),
                            ..default()
                        },
                        transform: Transform {
                            scale: Vec3::new(scale, scale, 0.),
                            translation: translate_to_cell_size(x, y, scale),
                            ..default()
                        },
                        ..default()
                    },
                    OnGameScreen,
                )
            })
            .collect::<Vec<_>>(),
    );
}

fn translate_to_cell_size(x: u8, y: u8, scale: f32) -> Vec3 {
    Vec3::new((x as f32 - 4.) * scale, (y as f32 - 4.) * scale, 1.)
}

fn clear_selected_number(game: &mut Game, selected_number: &SelectionQuery) {
    let Ok((x, y)) = selected_number.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    if !game.start.has(x, y) {
        game.current = game.current.unset(x, y);
    }
}

fn fill_selected_number(game: &mut Game, selected_number: &SelectionQuery, n: NonZeroU8) {
    if let Ok((x, y)) = selected_number
        .get_single()
        .map(|number| (number.0, number.1))
    {
        game.set(x, y, n);
    }
}

fn toggle_note(game: &mut Game, selected_number: &SelectionQuery, n: NonZeroU8) {
    let Ok((x, y)) = selected_number.get_single().map(|number| (number.0, number.1)) else {
        return;
    };

    game.notes.toggle(x, y, n);
}

fn move_selected_number(
    commands: &mut Commands,
    selected_number: &mut SelectionQuery,
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
    selected_number: &mut SelectionQuery,
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

fn button_actions(
    mut commands: Commands,
    query: Query<(&Interaction, &UiButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game: ResMut<Game>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    hint: Query<&Hint>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Clicked {
            match action {
                UiButtonAction::BackToMain => screen_state.set(ScreenState::MainMenu),
                UiButtonAction::Hint => give_hint(&mut commands, game.as_mut(), &hint),
            }
        }
    }
}

fn give_hint(commands: &mut Commands, game: &mut Game, hint: &Query<&Hint>) {
    if let Ok(Hint(x, y)) = hint.get_single() {
        if let Some(n) = game.solution.get(*x, *y) {
            game.set(*x, *y, n);
        }
    } else if let Some(sudoku::Hint { x, y }) = game.current.get_hint() {
        commands.spawn((
            Hint(x, y),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.2, 0.9, 0.0, 0.35),
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ));
    }
}
