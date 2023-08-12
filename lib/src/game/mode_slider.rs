use crate::{constants::*, pointer_query::*, ui::*, utils::TransformExt, Fonts, Images};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::*};
use bevy_tweening::{Animator, EaseFunction, Lens, Tween};
use std::time::Duration;

const ACTIVE_KNOB_Z: f32 = INACTIVE_KNOB_Z + 2.;
const INACTIVE_KNOB_Z: f32 = 2.;
const ANIMATION_DURATION: Duration = Duration::from_millis(300);

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

pub fn build_mode_slider(
    parent: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    images: &Images,
) {
    parent.with_children(|parent| {
        parent
            .spawn((
                ModeSlider { active: false },
                FlexBundle::new(
                    FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(30.))
                        .with_margin(Size::new(Val::None, Val::Vmin(4.5))),
                    FlexContainerStyle::column(),
                ),
            ))
            .with_children(|column| {
                column
                    .spawn(FlexBundle::new(
                        FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(9.)),
                        FlexContainerStyle::row(),
                    ))
                    .with_children(|row| {
                        build_background(row, images);
                        build_knobs(row, meshes, materials);
                    });

                column
                    .spawn(FlexBundle::new(
                        FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(21.)),
                        FlexContainerStyle::row(),
                    ))
                    .with_children(|row| {
                        build_labels(row, fonts);
                    });
            });
    });
}

fn build_background(row: &mut ChildBuilder, images: &Images) {
    row.spawn((
        FlexItemBundle::from_style(
            FlexItemStyle::fixed_size(Val::Percent(100.), Val::CrossPercent(9.2))
                .with_fixed_aspect_ratio()
                .without_occupying_space()
                .with_margin(Size::new(Val::None, Val::CrossPercent(1.5)))
                .with_transform(Transform::from_2d_scale(1. / 1282., 1. / 118.)),
        ),
        SpriteBundle {
            texture: images.mode_slider.clone(),
            ..default()
        },
    ));
}

fn build_knobs(
    row: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    build_knob(
        row,
        meshes,
        materials,
        ModeSliderKnob,
        COLOR_TOGGLE_ON,
        0.,
        ACTIVE_KNOB_Z,
    );
    build_knob(
        row,
        meshes,
        materials,
        OppositeSliderKnob,
        COLOR_BOARD_LINE_THIN,
        0.9,
        INACTIVE_KNOB_Z,
    );
}

fn build_knob(
    row: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    knob: impl Component,
    color: Color,
    x: f32,
    z_index: f32,
) {
    row.spawn((
        knob,
        FlexLeafBundle::from_style(
            FlexItemStyle::fixed_size(Val::CrossPercent(100.), Val::Percent(100.))
                .without_occupying_space()
                .with_transform(Transform::from_translation(Vec3::new(x, 0., z_index))),
        ),
    ))
    .with_children(|knob_container| {
        knob_container.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(0.6).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        },));
    });
}

fn build_labels(row: &mut ChildBuilder, fonts: &Fonts) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 48.,
        color: COLOR_MAIN_DARKER,
    };

    row.spawn(FlexBundle::new(
        FlexItemStyle::available_size(),
        FlexContainerStyle::default(),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Normal\nmode", text_style.clone()))
                .with_anchor(Anchor::CenterLeft),
        );
    });

    row.spawn(FlexBundle::new(
        FlexItemStyle::available_size(),
        FlexContainerStyle::default(),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Notes\nmode", text_style))
                .with_anchor(Anchor::CenterRight),
        );
    });
}

pub fn slider_interaction(
    mut commands: Commands,
    mut slider_query: Query<(&ComputedPosition, &mut ModeSlider)>,
    mut next_state: ResMut<NextState<ModeState>>,
    state: Res<State<ModeState>>,
    knob_query: Query<(Entity, &ComputedPosition), (With<ModeSliderKnob>, Without<ModeSlider>)>,
    opposite_query: Query<Entity, (With<OppositeSliderKnob>, Without<ModeSlider>)>,
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

    let Ok((knob, knob_position)) = knob_query.get_single() else {
        return;
    };

    let mode = if position.x > slider_position.x + 0.5 * slider_position.width {
        ModeState::Notes
    } else {
        ModeState::Normal
    };
    if state.get() != &mode {
        next_state.set(mode);
    }

    let knob_width = knob_position.width / slider_position.width;
    let width = 1. - knob_width;
    let knob_start =
        (position.x - slider_position.x - 0.5 * knob_position.width) / slider_position.width;
    let knob_end = match mode {
        ModeState::Normal => 0.,
        ModeState::Notes => width,
    };
    let animator = Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        ANIMATION_DURATION,
        TransformTranslateKnobLens {
            start: knob_start.clamp(0., 0.9),
            end: knob_end,
            center: 0.5 * width,
        },
    ));

    commands.entity(knob).insert(animator);

    if state.get() != &mode {
        if let Ok(opposite) = opposite_query.get_single() {
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
        let scale = 0.25 + 0.75 * distance_from_center;
        target.transform.scale = Vec3::new(scale, scale, 1.);
        target.transform.translation.x = x;
    }
}
