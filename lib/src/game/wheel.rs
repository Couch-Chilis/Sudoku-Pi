use super::mode_slider::*;
use super::{fill_number, get_board_x_and_y, toggle_note, Board, InputKind, Note, Selection};
use crate::{constants::*, pointer_query::*, ui::*, utils::*};
use crate::{ComputedPosition, Game, GameTimer, Images, ScreenSizing, ScreenState, Settings};
use bevy::prelude::*;
use std::{f32::consts::PI, num::NonZeroU8};

const MAX_RADIUS: f32 = 0.6;
const MAX_RADIUS_IPAD: f32 = 0.4;
const WHEEL_SIZE: f32 = 400.;
const WHEEL_Z: f32 = 10.;
pub const NOTES_MODE_OPEN_DELAY: f32 = 0.6;

#[derive(Component, Default)]
pub struct Wheel {
    pub cell: (u8, u8),
    pub center_position: Vec2,
    pub start_position: Vec2,
    pub current_position: Vec2,
    pub is_open: bool,
    pub mode: ModeState,
    pub spawn_timer: f32,
    pub selected_number: Option<NonZeroU8>,
    pub slice_timer: f32,
}

#[derive(Component)]
pub struct Slice;

#[derive(Component)]
pub struct DisabledSlice(NonZeroU8);

#[derive(Component)]
pub struct TopLabel;

#[derive(Component)]
pub struct TopLabelText;

#[derive(Default, Resource)]
pub struct ActiveSliceHandles {
    slices: [Handle<Image>; 9],
}

impl ActiveSliceHandles {
    pub fn load(images: &Images) -> Self {
        Self {
            slices: [
                images.slice_active_1.handle.clone(),
                images.slice_active_2.handle.clone(),
                images.slice_active_3.handle.clone(),
                images.slice_active_4.handle.clone(),
                images.slice_active_5.handle.clone(),
                images.slice_active_6.handle.clone(),
                images.slice_active_7.handle.clone(),
                images.slice_active_8.handle.clone(),
                images.slice_active_9.handle.clone(),
            ],
        }
    }

    fn for_number(&self, n: NonZeroU8) -> Handle<Image> {
        self.slices[n.get() as usize - 1].clone()
    }
}

pub fn wheel(screen: ScreenState) -> impl FnOnce(&Props, &mut ChildBuilder) {
    move |props, cb| {
        cb.spawn((
            Wheel::default(),
            screen,
            SpriteBundle {
                texture: props.resources.images.wheel.handle.clone(),
                transform: Transform::from_2d_scale(0., 0.),
                ..default()
            },
        ))
        .with_children(|wheel| {
            for (i, disabled_slice) in get_disabled_slice_handles(props.resources.images)
                .into_iter()
                .enumerate()
            {
                wheel.spawn((
                    DisabledSlice(NonZeroU8::new(i as u8 + 1).unwrap()),
                    SpriteBundle {
                        texture: disabled_slice.clone(),
                        transform: Transform::default_2d(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                ));
            }
        });

        cb.spawn((
            Slice,
            screen,
            SpriteBundle {
                transform: Transform::from_2d_scale(0., 0.),
                ..default()
            },
        ));

        let label_text_style = TextStyle {
            font: props.resources.fonts.medium.clone(),
            font_size: 50.,
            color: COLOR_WHEEL_TOP_TEXT,
        };

        cb.spawn((
            TopLabel,
            screen,
            SpriteBundle {
                texture: props.resources.images.top_label.handle.clone(),
                transform: Transform::from_2d_scale(0., 0.),
                ..default()
            },
        ))
        .with_children(|center_label| {
            center_label.spawn((
                TopLabelText,
                Text2dBundle {
                    text: Text::from_section("", label_text_style),
                    transform: Transform::from_translation(Vec3::new(0., 8., 1.)),
                    ..default()
                },
            ));
        });
    }
}

fn get_disabled_slice_handles(images: &Images) -> [&Handle<Image>; 9] {
    [
        &images.slice_disabled_1.handle,
        &images.slice_disabled_2.handle,
        &images.slice_disabled_3.handle,
        &images.slice_disabled_4.handle,
        &images.slice_disabled_5.handle,
        &images.slice_disabled_6.handle,
        &images.slice_disabled_7.handle,
        &images.slice_disabled_8.handle,
        &images.slice_disabled_9.handle,
    ]
}

pub fn on_wheel_input(
    mut wheel: Query<(&mut Wheel, &ScreenState)>,
    mut game: ResMut<Game>,
    mut selection: ResMut<Selection>,
    mut notes: Query<&mut Note>,
    mut timer: ResMut<GameTimer>,
    mode: Res<State<ModeState>>,
    pointer_query: PointerQuery,
    board: Query<(&ComputedPosition, &ScreenState), With<Board>>,
    screen: Res<State<ScreenState>>,
    settings: Res<Settings>,
) {
    use ScreenState::*;
    let screen = screen.get();
    if !matches!(screen, Game | LearnNumbers | LearnNotes) {
        return;
    }

    let Some((input_kind, position)) = pointer_query.get_changed_input_with_position() else {
        return;
    };

    let Some(board_position) = board
        .iter()
        .find_map(|board| (board.1 == screen).then_some(board.0))
    else {
        return;
    };

    let Some(translation) = get_board_translation(board_position, position) else {
        return;
    };

    let Some(mut wheel) = wheel
        .iter_mut()
        .find_map(|wheel| (wheel.1 == screen).then_some(wheel.0))
    else {
        return;
    };

    wheel.current_position = translation;

    match input_kind {
        InputKind::Press => {
            if let Some((x, y)) = get_board_x_and_y(board_position, position) {
                let should_open = if settings.show_mistakes {
                    !game.current.has(x, y)
                } else {
                    !game.start.has(x, y)
                };

                if should_open {
                    wheel.center_position = translation;
                    wheel.start_position = translation;
                    wheel.cell = (x, y);
                    wheel.spawn_timer = 0.;
                    wheel.is_open = true;
                    wheel.selected_number = None;
                    wheel.mode = *mode.get();
                }
            }
        }
        InputKind::PressedMovement => {
            if wheel.is_open {
                // In notes mode, the wheel only opens after a long press,
                // so we cancel opening if there's movement during the delay.
                if wheel.mode == ModeState::Notes && wheel.spawn_timer < NOTES_MODE_OPEN_DELAY {
                    if wheel.start_position.distance(wheel.current_position) > 0.01 {
                        wheel.is_open = false;
                    }
                    return;
                }

                let selected_number = get_selected_number(&wheel);
                let may_select_number = !settings.enable_wheel_aid
                    || match selected_number {
                        Some(n) => {
                            let sudoku = if settings.show_mistakes {
                                &game.current
                            } else {
                                &game.start
                            };
                            sudoku.may_set(wheel.cell.0, wheel.cell.1, n)
                        }
                        None => true, // It should always be allowed to deselect.
                    };
                if may_select_number && selected_number != wheel.selected_number {
                    wheel.selected_number = selected_number;
                    wheel.slice_timer = 0.;
                }
            }
        }
        InputKind::Release => {
            if wheel.is_open {
                wheel.is_open = false;

                if let Some(n) = wheel.selected_number {
                    let (x, y) = wheel.cell;
                    match wheel.mode {
                        ModeState::Normal => {
                            fill_number(
                                &mut game,
                                &mut timer,
                                &mut selection,
                                &mut notes,
                                settings.show_mistakes,
                                false,
                                x,
                                y,
                                n,
                            );
                        }
                        ModeState::Notes => {
                            if selection.selected_cell != Some((x, y)) {
                                selection.selected_cell = Some((x, y));
                            }

                            toggle_note(&mut game, &mut selection, n);
                        }
                    }
                }
            }
        }
    }
}

pub fn on_wheel_timer(
    mut wheel: Query<&mut Wheel>,
    time: Res<Time>,
    screen: Res<State<ScreenState>>,
) {
    use ScreenState::*;
    let screen = screen.get();
    if !matches!(screen, Game | LearnNumbers | LearnNotes) {
        return;
    }

    for mut wheel in &mut wheel {
        if wheel.is_open {
            wheel.spawn_timer += time.delta().as_secs_f32();

            if wheel.selected_number.is_some() && wheel.slice_timer < 0.25 {
                wheel.slice_timer += time.delta().as_secs_f32();
            }
        }
    }
}

pub fn render_wheel(
    mut wheel: Query<
        (&mut Transform, &mut Wheel, &ScreenState),
        (Changed<Wheel>, Without<Slice>, Without<TopLabel>),
    >,
    mut slice: Query<
        (&mut Transform, &mut Handle<Image>, &ScreenState),
        (With<Slice>, Without<TopLabel>),
    >,
    mut top_label: Query<
        (&mut Transform, &ScreenState),
        (With<TopLabel>, Without<Wheel>, Without<Slice>),
    >,
    mut top_label_text: Query<&mut Text, With<TopLabelText>>,
    active_slice_handles: Res<ActiveSliceHandles>,
    screen: Res<State<ScreenState>>,
    screen_sizing: Res<ScreenSizing>,
) {
    let screen = screen.get();

    let Some((mut wheel_transform, mut wheel)) = wheel
        .iter_mut()
        .find_map(|wheel| (wheel.2 == screen).then_some((wheel.0, wheel.1)))
    else {
        return;
    };

    let Some((mut slice_transform, mut slice_texture)) = slice
        .iter_mut()
        .find_map(|slice| (slice.2 == screen).then_some((slice.0, slice.1)))
    else {
        return;
    };

    let Some(mut top_label_transform) = top_label
        .iter_mut()
        .find_map(|top_label| (top_label.1 == screen).then_some(top_label.0))
    else {
        return;
    };

    if !wheel.is_open
        || (wheel.mode == ModeState::Notes && wheel.spawn_timer < NOTES_MODE_OPEN_DELAY)
    {
        *wheel_transform = Transform::from_2d_scale(0., 0.);
        *slice_transform = Transform::from_2d_scale(0., 0.);
        *top_label_transform = Transform::from_2d_scale(0., 0.);
        return;
    }

    let radius = get_radius(&screen_sizing, &wheel);
    let center_position = get_wheel_center(&screen_sizing, &wheel, radius);
    if wheel.center_position != center_position {
        wheel.center_position = center_position;
    }

    let Vec2 { x: cx, y: cy } = center_position;

    wheel_transform.translation = Vec3::new(cx, cy, WHEEL_Z);
    wheel_transform.scale = Vec3::new(radius / WHEEL_SIZE, radius / WHEEL_SIZE, 1.);

    if let Some(n) = wheel.selected_number {
        let bounce = 1.
            + 0.1
                * ((wheel.slice_timer * 100.).powi(2) * 0.0016 * PI)
                    .sin()
                    .max(0.);
        let scale = bounce * radius / WHEEL_SIZE;

        *slice_texture = active_slice_handles.for_number(n);
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

pub fn render_disabled_wheel_slices(
    mut disabled_slices: Query<(&DisabledSlice, &mut Visibility)>,
    game: Res<Game>,
    settings: Res<Settings>,
    wheel: Query<&Wheel, Changed<Wheel>>,
) {
    for wheel in &wheel {
        let (x, y) = wheel.cell;

        for (DisabledSlice(n), mut visibility) in &mut disabled_slices {
            let is_disabled = settings.enable_wheel_aid && !game.current.may_set(x, y, *n);
            *visibility = if is_disabled {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn get_board_translation(board_position: &ComputedPosition, cursor_position: Vec2) -> Option<Vec2> {
    let Vec2 { x, y } = cursor_position;

    let board_x = (x - board_position.x) / board_position.width - 0.5;
    let board_y = (y - board_position.y) / board_position.height - 0.5;
    Some(Vec2::new(board_x, board_y))
}

fn get_radius(screen_sizing: &ScreenSizing, wheel: &Wheel) -> f32 {
    let mut spawn_timer = wheel.spawn_timer;
    if wheel.mode == ModeState::Notes {
        spawn_timer = (spawn_timer - NOTES_MODE_OPEN_DELAY).max(0.);
    }

    let finger_radius = 2.5 * wheel.start_position.distance(wheel.current_position);
    let time_radius = (spawn_timer * 40.).powi(2) / 10.;
    finger_radius
        .max(time_radius)
        .min(if screen_sizing.is_ipad {
            MAX_RADIUS_IPAD
        } else {
            MAX_RADIUS
        })
}

/// Returns the X and Y coordinates of the center position of the wheel.
///
/// The center position is usually the position where the press started, but it
/// maybe adjusted to avoid the wheel from going outside the screen dimensions.
fn get_wheel_center(screen_sizing: &ScreenSizing, wheel: &Wheel, radius: f32) -> Vec2 {
    let mut cx = wheel.start_position.x;
    let cy = wheel.start_position.y;

    if !screen_sizing.is_ipad {
        let overflow_ratio = 0.9;

        if cx + radius > overflow_ratio {
            cx = overflow_ratio - radius;
        } else if cx - radius < -overflow_ratio {
            cx = -overflow_ratio + radius;
        }
    }

    Vec2::new(cx, cy)
}

fn get_selected_number(wheel: &Wheel) -> Option<NonZeroU8> {
    let center = wheel.center_position;

    let current_x = wheel.current_position.x;
    let current_y = wheel.current_position.y;
    let angle = (current_y - center.y).atan2(current_x - center.x);

    let diff_x = (current_x - center.x).abs();
    let diff_y = (current_y - center.y).abs();
    let touch_radius = (diff_x * diff_x + diff_y * diff_y).sqrt();

    if touch_radius > 0.08 && touch_radius < 0.5 {
        let n = (10.75 - (angle / PI * 4.5)).round() as u8 % 9 + 1;
        Some(NonZeroU8::new(n).unwrap())
    } else {
        None
    }
}
