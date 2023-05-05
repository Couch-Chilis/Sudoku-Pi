use super::{board_builder::Board, fill_number, get_board_x_and_y};
use crate::{utils::*, ComputedPosition, Game, GameTimer};
use bevy::{ecs::system::EntityCommands, prelude::*, time::Stopwatch, window::PrimaryWindow};
use std::{f32::consts::PI, num::NonZeroU8};

const MAX_RADIUS: f32 = 0.6;
const WHEEL_SIZE: f32 = 400.;

#[derive(Component, Default)]
pub struct Wheel {
    cell: (u8, u8),
    start_position: Vec2,
    current_position: Vec2,
    is_pressed: bool,
    stopwatch: Stopwatch,
}

pub fn init_wheel(board: &mut EntityCommands, asset_server: &AssetServer) {
    board.with_children(|board| {
        board.spawn((
            Wheel::default(),
            SpriteBundle {
                texture: asset_server.load("wheel.png"),
                transform: Transform::from_2d_scale(0., 0.),
                ..default()
            },
        ));
    });
}

pub fn on_wheel_input(
    mut wheel: Query<&mut Wheel>,
    mut game: ResMut<Game>,
    mut timer: ResMut<GameTimer>,
    board: Query<&ComputedPosition, With<Board>>,
    buttons: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if !buttons.is_changed() && !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_position) = primary_window.get_single().ok().and_then(|window| window.cursor_position()) else {
        return;
    };

    let Ok(board_position) = board.get_single() else {
        return;
    };

    let Some(translation) = get_board_translation(board_position, cursor_position) else {
        return;
    };

    let Ok(mut wheel) = wheel.get_single_mut() else {
        return;
    };

    wheel.current_position = translation;

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cell_xy) = get_board_x_and_y(board_position, cursor_position) {
            wheel.start_position = translation;
            wheel.cell = cell_xy;
            wheel.stopwatch.reset();
            wheel.is_pressed = true;
        }
    } else if buttons.just_released(MouseButton::Left) {
        wheel.is_pressed = false;

        let radius = get_radius(&wheel);
        if let Some(selected_number) = get_wheel_data(&wheel, radius).selected_number {
            let (x, y) = wheel.cell;
            fill_number(&mut game, &mut timer, x, y, selected_number);
        }
    }
}

pub fn on_wheel_timer(mut wheel: Query<&mut Wheel>, time: Res<Time>) {
    for mut wheel in &mut wheel {
        if wheel.is_pressed {
            wheel.stopwatch.tick(time.delta());
        }
    }
}

pub fn render_wheel(mut wheel: Query<(&mut Transform, &Wheel), Changed<Wheel>>) {
    let Ok((mut transform, wheel)) = wheel.get_single_mut() else {
        return;
    };

    if !wheel.is_pressed {
        *transform = Transform::from_2d_scale(0., 0.);
        return;
    }

    let radius = get_radius(wheel);
    let WheelData { cx, cy, .. } = get_wheel_data(wheel, radius);

    transform.translation = Vec3::new(cx, cy, 10.);
    transform.scale = Vec3::new(radius / WHEEL_SIZE, radius / WHEEL_SIZE, 1.);
}

fn get_board_translation(board_position: &ComputedPosition, cursor_position: Vec2) -> Option<Vec2> {
    let Vec2 { x, y } = cursor_position;

    let board_x = (x - board_position.x) / board_position.width - 0.5;
    let board_y = (y - board_position.y) / board_position.height - 0.5;
    Some(Vec2::new(board_x, board_y))
}

fn get_radius(wheel: &Wheel) -> f32 {
    let finger_radius = 2.5 * wheel.start_position.distance(wheel.current_position);
    let time_radius = (wheel.stopwatch.elapsed_secs() * 50.).powi(2) / 10.;
    finger_radius.max(time_radius).min(MAX_RADIUS)
}

/// Contains the X and Y coordinates of the center position of the wheel, as
/// well as which number has been selected.
///
/// The center position is usually the position where the press started, but it
/// maybe adjusted to avoid the wheel from going outside the screen dimensions.
struct WheelData {
    cx: f32,
    cy: f32,
    selected_number: Option<NonZeroU8>,
}

fn get_wheel_data(wheel: &Wheel, radius: f32) -> WheelData {
    let overflow_ratio = 1.1;

    let mut cx = wheel.start_position.x;
    if cx + radius > overflow_ratio {
        cx = overflow_ratio - radius;
    } else if cx - radius < -overflow_ratio {
        cx = -overflow_ratio + radius;
    }

    let mut cy = wheel.start_position.y;
    if cy + radius > overflow_ratio {
        cy = overflow_ratio - radius;
    } else if cy - radius < -overflow_ratio {
        cy = -overflow_ratio + radius;
    }

    let current_x = wheel.current_position.x;
    let current_y = wheel.current_position.y;
    let angle = (current_y - cy).atan2(current_x - cx);

    let diff_x = (current_x - cx).abs();
    let diff_y = (current_y - cy).abs();
    let touch_radius = (diff_x * diff_x + diff_y * diff_y).sqrt();
    let selected_number = if touch_radius > 0.05 && touch_radius < 0.5 {
        let n = (11.25 - ((angle + 1.047) / PI * 4.5)).round() as u8 % 9 + 1;
        Some(NonZeroU8::new(n).unwrap())
    } else {
        None
    };

    WheelData {
        cx,
        cy,
        selected_number,
    }
}
