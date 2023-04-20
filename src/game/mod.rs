mod board_builder;
mod board_line_bundle;
mod board_numbers;

use self::{board_builder::build_board, board_numbers::fill_numbers};
use crate::{despawn, sudoku::Game, ScreenState, WindowSize};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::default()).add_systems((
            board_setup.in_schedule(OnEnter(ScreenState::Game)),
            despawn::<OnGameScreen>.in_schedule(OnExit(ScreenState::Game)),
            mouse_button_input.run_if(in_state(ScreenState::Game)),
            render_selection.run_if(in_state(ScreenState::Game)),
        ));
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Number;

#[derive(Component)]
struct SelectedNumber(u8, u8);

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    window_size: Res<WindowSize>,
) {
    build_board(&mut commands, &window_size);
    fill_numbers(&mut commands, &asset_server, &game, &window_size);
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
                        transform: Transform {
                            scale: Vec3::new(
                                10. * window_size.vmin_scale,
                                10. * window_size.vmin_scale,
                                0.,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    OnGameScreen,
                ));
            }
        }
    }
}

fn render_selection(
    mut selected_number_query: Query<(&SelectedNumber, &mut Transform), Changed<SelectedNumber>>,
    window_size: Res<WindowSize>,
) {
    for (selected_number, mut transform) in &mut selected_number_query {
        let scale = 10. * window_size.vmin_scale;
        let SelectedNumber(x, y) = selected_number;
        transform.translation = Vec3::new((*x as f32 - 4.) * scale, (*y as f32 - 4.) * scale, 1.);
    }
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
