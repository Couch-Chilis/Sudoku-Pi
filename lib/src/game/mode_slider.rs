use crate::{constants::*, pointer_query::*, ui::*};
use crate::{utils::TransformExt, ResourceBag, ScreenSizing};
use bevy::{prelude::*, sprite::*};
use bevy_tweening::{Animator, EaseFunction, Lens, Tween};
use std::time::Duration;

const ACTIVE_KNOB_Z: f32 = INACTIVE_KNOB_Z + 2.;
const INACTIVE_KNOB_Z: f32 = 2.;
const ANIMATION_DURATION: Duration = Duration::from_millis(300);
const KNOB_SCALE: f32 = 0.013;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ModeState {
    #[default]
    Normal,
    Notes,
}

#[derive(Component)]
pub struct ModeSlider {
    active: bool,
}

#[derive(Component)]
pub struct ModeSliderKnob;

#[derive(Component)]
pub struct OppositeSliderKnob;

pub fn build_mode_slider(cb: &mut ChildBuilder, resources: &ResourceBag) {
    if resources.screen_sizing.is_ipad {
        cb.spawn((
            ModeSlider { active: false },
            FlexBundle::new(
                FlexItemStyle::preferred_size(Val::Vmin(80.), Val::Pixel(105))
                    .with_margin(Size::new(Val::None, Val::Pixel(15))),
                FlexContainerStyle::row(),
            ),
        ))
        .with_children(|row| {
            build_label(row, resources, "Normal\nmode", Anchor::CenterLeft);

            row.spawn(FlexBundle::new(
                FlexItemStyle::fixed_size(Val::Percent(66.), Val::CrossPercent(6.))
                    .with_fixed_aspect_ratio(),
                FlexContainerStyle::row(),
            ))
            .with_children(|row| {
                build_background(row, resources);
                build_knobs(row, resources);
            });

            build_label(row, resources, "Notes\nmode", Anchor::CenterRight);
        });
    } else {
        cb.spawn((
            ModeSlider { active: false },
            FlexBundle::new(
                FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Pixel(105))
                    .with_margin(Size::new(Val::None, Val::Pixel(15))),
                FlexContainerStyle::column(),
            ),
        ))
        .with_children(|column| {
            column
                .spawn(FlexBundle::new(
                    FlexItemStyle::fixed_size(Val::Percent(100.), Val::CrossPercent(9.2))
                        .with_fixed_aspect_ratio(),
                    FlexContainerStyle::row(),
                ))
                .with_children(|row| {
                    build_background(row, resources);
                    build_knobs(row, resources);
                });

            column
                .spawn(FlexBundle::new(
                    FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(70)),
                    FlexContainerStyle::row(),
                ))
                .with_children(|row| {
                    build_label(row, resources, "Normal\nmode", Anchor::CenterLeft);
                    build_label(row, resources, "Notes\nmode", Anchor::CenterRight);
                });
        });
    }
}

fn build_background(row: &mut ChildBuilder, resources: &ResourceBag) {
    row.spawn((
        FlexItemBundle::from_style(
            FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(100.))
                .without_occupying_space()
                .with_margin(Size::new(Val::None, Val::CrossPercent(1.5)))
                .with_transform(Transform::from_2d_scale(1. / 1282., 1. / 118.)),
        ),
        SpriteBundle {
            texture: resources.images.mode_slider.clone(),
            ..default()
        },
    ));
}

fn build_knobs(row: &mut ChildBuilder, resources: &ResourceBag) {
    build_knob(
        row,
        &resources.images.pop_dark_circle,
        ModeSliderKnob,
        0.,
        ACTIVE_KNOB_Z,
    );
    build_knob(
        row,
        &resources.images.board_line_thin_circle,
        OppositeSliderKnob,
        0.91,
        INACTIVE_KNOB_Z,
    );
}

fn build_knob(
    row: &mut ChildBuilder,
    image: &Handle<Image>,
    knob: impl Component,
    x: f32,
    z_index: f32,
) {
    row.spawn((
        knob,
        FlexItemBundle::from_style(
            FlexItemStyle::fixed_size(Val::CrossPercent(100.), Val::Percent(100.))
                .without_occupying_space()
                .with_transform(Transform {
                    scale: Vec3::new(KNOB_SCALE, KNOB_SCALE, 1.),
                    translation: Vec3::new(x, 0., z_index),
                    ..default()
                }),
        ),
        SpriteBundle {
            texture: image.clone(),
            ..default()
        },
    ));
}

fn build_label(row: &mut ChildBuilder, resources: &ResourceBag, text: &str, anchor: Anchor) {
    let text_style = TextStyle {
        font: resources.fonts.medium.clone(),
        font_size: if resources.screen_sizing.is_ipad {
            64.
        } else {
            48.
        },
        color: COLOR_MAIN_DARKER,
    };

    row.spawn(FlexBundle::new(
        FlexItemStyle::available_size(),
        FlexContainerStyle::default(),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section(text, text_style.clone()))
                .with_anchor(anchor),
        );
    });
}

pub fn slider_interaction(
    mut next_state: ResMut<NextState<ModeState>>,
    mut slider_query: Query<(&ComputedPosition, &mut ModeSlider)>,
    state: Res<State<ModeState>>,
    pointer_query: PointerQuery,
) {
    let Some((input, position)) = pointer_query.get_changed_input_with_position() else {
        return;
    };

    let Ok((slider_position, mut mode_slider)) = slider_query.get_single_mut() else {
        return;
    };

    if input == InputKind::Press && slider_position.contains(position) {
        mode_slider.active = true;
    } else if input == InputKind::Release {
        mode_slider.active = false;
        return;
    } else if !mode_slider.active {
        return;
    }

    let mode = if position.x > slider_position.x + 0.5 * slider_position.width {
        ModeState::Notes
    } else {
        ModeState::Normal
    };
    if state.get() != &mode {
        next_state.set(mode);
    }
}

pub fn render_slider_knobs(
    mut commands: Commands,
    slider_query: Query<(&ModeSlider, &ComputedPosition)>,
    mode: Res<State<ModeState>>,
    knob_query: Query<(Entity, &ComputedPosition), With<ModeSliderKnob>>,
    opposite_knob_query: Query<Entity, With<OppositeSliderKnob>>,
    pointer_query: PointerQuery,
    screen_sizing: Res<ScreenSizing>,
) {
    let Ok((slider_active, slider_position)) = slider_query
        .get_single()
        .map(|(slider, position)| (slider.active, position))
    else {
        return;
    };

    if mode.is_added() || !mode.is_changed() && !slider_active {
        return;
    }

    let Ok((knob, knob_position)) = knob_query.get_single() else {
        return;
    };

    let slider_width = if screen_sizing.is_ipad {
        0.66 * slider_position.width
    } else {
        slider_position.width
    };
    let slider_x = if screen_sizing.is_ipad {
        slider_position.x + 0.17 * slider_position.width
    } else {
        slider_position.x
    };

    let knob_width = knob_position.width / slider_width;
    let width = 1. - knob_width;
    let knob_start = if slider_active {
        let Some((_, position)) = pointer_query.get_changed_input_with_position() else {
            return;
        };
        (position.x - slider_x - 0.5 * knob_position.width) / slider_width
    } else {
        match mode.get() {
            ModeState::Normal => width - knob_width,
            ModeState::Notes => -knob_width,
        }
    };

    let knob_end = match mode.get() {
        ModeState::Normal => 0.,
        ModeState::Notes => width,
    };

    let animator = Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        ANIMATION_DURATION,
        TransformTranslateKnobLens {
            start: knob_start.clamp(0., 0.91),
            end: knob_end,
            center: 0.5 * width,
        },
    ));

    commands.entity(knob).insert(animator);

    if mode.is_changed() {
        if let Ok(opposite) = opposite_knob_query.get_single() {
            let animator = Animator::new(Tween::new(
                EaseFunction::QuadraticInOut,
                ANIMATION_DURATION,
                TransformTranslateKnobLens {
                    start: knob_end,
                    end: width - knob_end,
                    center: 0.5 * width,
                },
            ));

            commands.entity(opposite).insert(animator);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TransformTranslateKnobLens {
    pub start: f32,
    pub end: f32,
    pub center: f32,
}

impl Lens<FlexItemStyle> for TransformTranslateKnobLens {
    fn lerp(&mut self, target: &mut FlexItemStyle, ratio: f32) {
        let x = self.start + (self.end - self.start) * ratio;
        let distance_from_center = ((x - self.center) / self.center).abs();
        let scale = (0.25 + 0.75 * distance_from_center) * KNOB_SCALE;
        target.transform.scale = Vec3::new(scale, scale, 1.);
        target.transform.translation.x = x;
    }
}
