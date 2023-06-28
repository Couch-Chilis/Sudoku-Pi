use super::{fill_number, get_board_x_and_y, Board, InputKind};
use crate::{
    constants::*, settings::Settings, utils::*, ComputedPosition, Fonts, Game, GameTimer, Images,
};
use bevy::{ecs::system::EntityCommands, prelude::*, time::Stopwatch};
use std::{f32::consts::PI, num::NonZeroU8};

const MAX_RADIUS: f32 = 0.6;
const WHEEL_SIZE: f32 = 400.;
const WHEEL_Z: f32 = 10.;

#[derive(Component, Default)]
pub struct Wheel {
    cell: (u8, u8),
    start_position: Vec2,
    current_position: Vec2,
    is_open: bool,
    spawn_timer: Stopwatch,
    selected_number: Option<NonZeroU8>,
    slice_timer: Stopwatch,
}

#[derive(Component)]
pub struct Slice;

#[derive(Component)]
pub struct TopLabel;

#[derive(Component)]
pub struct TopLabelText;

#[derive(Default, Resource)]
pub struct SliceHandles {
    slices: [Handle<Image>; 9],
}

impl SliceHandles {
    pub fn load(images: &Images) -> Self {
        Self {
            slices: [
                images.slice_1.clone(),
                images.slice_2.clone(),
                images.slice_3.clone(),
                images.slice_4.clone(),
                images.slice_5.clone(),
                images.slice_6.clone(),
                images.slice_7.clone(),
                images.slice_8.clone(),
                images.slice_9.clone(),
            ],
        }
    }

    fn for_number(&self, n: NonZeroU8) -> Handle<Image> {
        self.slices[n.get() as usize - 1].clone()
    }
}

pub fn init_wheel(board: &mut EntityCommands, images: &Images, fonts: &Fonts) {
    board.with_children(|board| {
        board.spawn((
            Wheel::default(),
            SpriteBundle {
                texture: images.wheel.clone(),
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

        let label_text_style = TextStyle {
            font: fonts.medium.clone(),
            font_size: 40.,
            color: COLOR_WHEEL_TOP_TEXT,
        };

        board
            .spawn((
                TopLabel,
                SpriteBundle {
                    texture: images.top_label.clone(),
                    transform: Transform::from_2d_scale(0., 0.),
                    ..default()
                },
            ))
            .with_children(|center_label| {
                center_label.spawn((
                    TopLabelText,
                    Text2dBundle {
                        text: Text::from_section("", label_text_style),
                        transform: Transform::default_2d(),
                        ..default()
                    },
                ));
            });
    });
}

pub fn on_wheel_input(
    mut wheel: Query<&mut Wheel>,
    mut game: ResMut<Game>,
    mut timer: ResMut<GameTimer>,
    input_kind: InputKind,
    position: Vec2,
    board: Query<&ComputedPosition, With<Board>>,
    settings: Res<Settings>,
) {
    let Ok(board_position) = board.get_single() else {
        return;
    };

    let Some(translation) = get_board_translation(board_position, position) else {
        return;
    };

    let Ok(mut wheel) = wheel.get_single_mut() else {
        return;
    };

    wheel.current_position = translation;

    match input_kind {
        InputKind::Press => {
            if let Some((x, y)) = get_board_x_and_y(board_position, position) {
                let should_open = if settings.show_mistakes {
                    !game.current.has(x, y) || game.current.get(x, y) != game.solution.get(x, y)
                } else {
                    !game.start.has(x, y)
                };

                if should_open {
                    wheel.start_position = translation;
                    wheel.cell = (x, y);
                    wheel.spawn_timer.reset();
                    wheel.is_open = true;
                    wheel.selected_number = None;
                }
            }
        }
        InputKind::PressedMovement => {
            if wheel.is_open {
                let radius = get_radius(&wheel);
                let selected_number = get_selected_number(&wheel, radius);
                if selected_number != wheel.selected_number {
                    wheel.selected_number = selected_number;
                    wheel.slice_timer.reset();
                }
            }
        }
        InputKind::Release => {
            if wheel.is_open {
                wheel.is_open = false;

                if let Some(selected_number) = wheel.selected_number {
                    let (x, y) = wheel.cell;
                    fill_number(&mut game, &mut timer, x, y, selected_number);
                }
            }
        }
    }
}

pub fn on_wheel_timer(mut wheel: Query<&mut Wheel>, time: Res<Time>) {
    for mut wheel in &mut wheel {
        if wheel.is_open {
            wheel.spawn_timer.tick(time.delta());

            if wheel.selected_number.is_some() && wheel.slice_timer.elapsed_secs() < 0.25 {
                wheel.slice_timer.tick(time.delta());
            }
        }
    }
}

pub fn render_wheel(
    mut wheel: Query<(&mut Transform, &Wheel), (Changed<Wheel>, Without<Slice>, Without<TopLabel>)>,
    mut slice: Query<(&mut Transform, &mut Handle<Image>), (With<Slice>, Without<TopLabel>)>,
    mut top_label: Query<&mut Transform, (With<TopLabel>, Without<Wheel>, Without<Slice>)>,
    mut top_label_text: Query<&mut Text, With<TopLabelText>>,
    slice_handles: Res<SliceHandles>,
) {
    let Ok((mut wheel_transform, wheel)) = wheel.get_single_mut() else {
        return;
    };

    let Ok((mut slice_transform, mut slice_texture)) = slice.get_single_mut() else {
        return;
    };

    let Ok(mut top_label_transform) = top_label.get_single_mut() else {
        return;
    };

    if !wheel.is_open {
        *wheel_transform = Transform::from_2d_scale(0., 0.);
        *slice_transform = Transform::from_2d_scale(0., 0.);
        *top_label_transform = Transform::from_2d_scale(0., 0.);
        return;
    }

    let radius = get_radius(wheel);
    let Vec2 { x: cx, y: cy } = get_wheel_center(wheel, radius);

    wheel_transform.translation = Vec3::new(cx, cy, WHEEL_Z);
    wheel_transform.scale = Vec3::new(radius / WHEEL_SIZE, radius / WHEEL_SIZE, 1.);

    if let Some(n) = wheel.selected_number {
        let bounce = 1.
            + 0.1
                * ((wheel.slice_timer.elapsed_secs() * 100.).powi(2) * 0.0016 * PI)
                    .sin()
                    .max(0.);
        let scale = bounce * radius / WHEEL_SIZE;

        *slice_texture = slice_handles.for_number(n);
        slice_transform.translation = Vec3::new(cx, cy, WHEEL_Z + 1.);
        slice_transform.scale = Vec3::new(scale, scale, 1.);

        top_label_transform.translation = Vec3::new(cx, cy + 0.66 * radius, WHEEL_Z);
        top_label_transform.scale = Vec3::new(radius / WHEEL_SIZE, radius / WHEEL_SIZE, 1.);

        for mut top_label_text in &mut top_label_text {
            top_label_text.sections[0].value = n.to_string();
        }
    } else {
        *slice_transform = Transform::from_2d_scale(0., 0.);
        *top_label_transform = Transform::from_2d_scale(0., 0.);
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

    if touch_radius > 0.08 && touch_radius < 0.5 {
        let n = (11.25 - ((angle + 1.047) / PI * 4.5)).round() as u8 % 9 + 1;
        Some(NonZeroU8::new(n).unwrap())
    } else {
        None
    }
}
