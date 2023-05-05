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
    spawn_timer: Stopwatch,
    selected_number: Option<NonZeroU8>,
    slice_timer: Stopwatch,
}

#[derive(Component)]
pub struct Slice;

#[derive(Default, Resource)]
pub struct SliceHandles {
    slices: [Handle<Image>; 9],
}

impl SliceHandles {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            slices: [
                asset_server.load("slice_1.png"),
                asset_server.load("slice_2.png"),
                asset_server.load("slice_3.png"),
                asset_server.load("slice_4.png"),
                asset_server.load("slice_5.png"),
                asset_server.load("slice_6.png"),
                asset_server.load("slice_7.png"),
                asset_server.load("slice_8.png"),
                asset_server.load("slice_9.png"),
            ],
        }
    }

    fn for_number(&self, n: NonZeroU8) -> Handle<Image> {
        self.slices[n.get() as usize - 1].clone()
    }
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

        board.spawn((
            Slice,
            SpriteBundle {
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
            wheel.spawn_timer.reset();
            wheel.is_pressed = true;
            wheel.selected_number = None;
        }
    } else if buttons.just_released(MouseButton::Left) {
        wheel.is_pressed = false;

        if let Some(selected_number) = wheel.selected_number {
            let (x, y) = wheel.cell;
            fill_number(&mut game, &mut timer, x, y, selected_number);
        }
    } else {
        let radius = get_radius(&wheel);
        let selected_number = get_selected_number(&wheel, radius);
        if selected_number != wheel.selected_number {
            wheel.selected_number = selected_number;
            wheel.slice_timer.reset();
        }
    }
}

pub fn on_wheel_timer(mut wheel: Query<&mut Wheel>, time: Res<Time>) {
    for mut wheel in &mut wheel {
        if wheel.is_pressed {
            wheel.spawn_timer.tick(time.delta());

            if wheel.selected_number.is_some() && wheel.slice_timer.elapsed_secs() < 0.25 {
                wheel.slice_timer.tick(time.delta());
            }
        }
    }
}

pub fn render_wheel(
    mut wheel: Query<(&mut Transform, &Wheel), (Changed<Wheel>, Without<Slice>)>,
    mut slice: Query<(&mut Transform, &mut Handle<Image>), With<Slice>>,
    slice_handles: Res<SliceHandles>,
) {
    let Ok((mut wheel_transform, wheel)) = wheel.get_single_mut() else {
        return;
    };

    let Ok((mut slice_transform, mut slice_texture)) = slice.get_single_mut() else {
        return;
    };

    if !wheel.is_pressed {
        *wheel_transform = Transform::from_2d_scale(0., 0.);
        *slice_transform = Transform::from_2d_scale(0., 0.);
        return;
    }

    let radius = get_radius(wheel);
    let Vec2 { x: cx, y: cy } = get_wheel_center(wheel, radius);

    wheel_transform.translation = Vec3::new(cx, cy, 10.);
    wheel_transform.scale = Vec3::new(radius / WHEEL_SIZE, radius / WHEEL_SIZE, 1.);

    if let Some(n) = wheel.selected_number {
        let bounce = 1.
            + 0.1
                * ((wheel.slice_timer.elapsed_secs() * 100.).powi(2) * 0.0016 * PI)
                    .sin()
                    .max(0.);
        let scale = bounce * radius / WHEEL_SIZE;

        *slice_texture = slice_handles.for_number(n);
        slice_transform.translation = Vec3::new(cx, cy, 11.);
        slice_transform.scale = Vec3::new(scale, scale, 1.);
    } else {
        *slice_transform = Transform::from_2d_scale(0., 0.);
    }
}

fn get_board_translation(board_position: &ComputedPosition, cursor_position: Vec2) -> Option<Vec2> {
    let Vec2 { x, y } = cursor_position;

    let board_x = (x - board_position.x) / board_position.width - 0.5;
    let board_y = (y - board_position.y) / board_position.height - 0.5;
    Some(Vec2::new(board_x, board_y))
}

fn get_radius(wheel: &Wheel) -> f32 {
    let finger_radius = 2.5 * wheel.start_position.distance(wheel.current_position);
    let time_radius = (wheel.spawn_timer.elapsed_secs() * 40.).powi(2) / 10.;
    finger_radius.max(time_radius).min(MAX_RADIUS)
}

/// Returns the X and Y coordinates of the center position of the wheel.
///
/// The center position is usually the position where the press started, but it
/// maybe adjusted to avoid the wheel from going outside the screen dimensions.
fn get_wheel_center(wheel: &Wheel, radius: f32) -> Vec2 {
    let overflow_ratio = 0.9;

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

    Vec2::new(cx, cy)
}

fn get_selected_number(wheel: &Wheel, radius: f32) -> Option<NonZeroU8> {
    let center = get_wheel_center(wheel, radius);

    let current_x = wheel.current_position.x;
    let current_y = wheel.current_position.y;
    let angle = (current_y - center.y).atan2(current_x - center.x);

    let diff_x = (current_x - center.x).abs();
    let diff_y = (current_y - center.y).abs();
    let touch_radius = (diff_x * diff_x + diff_y * diff_y).sqrt();
    let selected_number = if touch_radius > 0.08 && touch_radius < 0.5 {
        let n = (11.25 - ((angle + 1.047) / PI * 4.5)).round() as u8 % 9 + 1;
        Some(NonZeroU8::new(n).unwrap())
    } else {
        None
    };

    selected_number
}
