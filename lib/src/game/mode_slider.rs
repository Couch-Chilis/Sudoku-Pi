use super::Wheel;
use crate::{constants::*, pointer_query::*, ui::*, Fonts};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::*};
use bevy_tweening::{Animator, EaseFunction, Lens, Tween};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ModeState {
    #[default]
    Normal,
    Notes,
}

#[derive(Component)]
pub struct ModeSlider;

#[derive(Component)]
pub struct ModeSliderKnob;

pub fn build_mode_slider(
    parent: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
) {
    parent.with_children(|parent| {
        parent
            .spawn(FlexBundle::new(
                FlexContainerStyle::row().with_padding(Size::new(Val::None, Val::Percent(25.))),
                FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(9.))
                    .with_margin(Size::new(Val::None, Val::Vmin(4.5))),
            ))
            .with_children(|row| build_items(row, meshes, materials, fonts));
    });
}

fn build_items(
    row: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: COLOR_SECONDARY_BUTTON_TEXT,
    };

    // This is the "knob" of the slider and the container in which it slides.
    row.spawn((
        ModeSlider,
        FlexBundle::new(
            FlexContainerStyle::row(),
            FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(100.))
                .without_occupying_space(),
        ),
    ))
    .with_children(|slider| {
        slider
            .spawn((
                ModeSliderKnob,
                FlexLeafBundle::from_style(
                    FlexItemStyle::fixed_size(Val::CrossPercent(100.), Val::Percent(100.))
                        .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
                ),
            ))
            .with_children(|knob_container| {
                knob_container.spawn((
                    Toggle,
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(0.21).into()).into(),
                        material: materials.add(ColorMaterial::from(COLOR_TOGGLE_ON)),
                        transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                        ..default()
                    },
                ));
            });
    });

    let mut toggle_builder = ToggleBuilder::new(meshes, materials);
    toggle_builder.build_with_marker(row, (), false);

    row.spawn(FlexBundle::from_item_style(
        FlexItemStyle::minimum_size(Val::Vmin(25.), Val::Percent(100.))
            .with_transform(Transform::from_translation(Vec3::new(0., 0., 3.))),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Normal", text_style.clone()))
                .with_anchor(Anchor::CenterLeft),
        );
    });

    row.spawn(FlexBundle::from_item_style(
        FlexItemStyle::minimum_size(Val::Vmin(25.), Val::Percent(100.))
            .with_transform(Transform::from_translation(Vec3::new(0., 0., 3.))),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Notes", text_style))
                .with_anchor(Anchor::CenterRight),
        );
    });

    toggle_builder.build_with_marker(row, (), false);
}

pub fn slider_interaction(
    mut commands: Commands,
    slider_query: Query<&ComputedPosition, With<ModeSlider>>,
    mut next_state: ResMut<NextState<ModeState>>,
    knob_query: Query<(Entity, &ComputedPosition), (With<ModeSliderKnob>, Without<ModeSlider>)>,
    pointer_query: PointerQuery,
    wheel_query: Query<&Wheel>,
) {
    if wheel_query
        .get_single()
        .map(Wheel::is_open)
        .unwrap_or_default()
    {
        return;
    }

    let Some((_, position)) = pointer_query.get_changed_input_with_position() else {
        return;
    };

    let Ok(slider_position) = slider_query.get_single() else {
        return;
    };

    if !slider_position.contains(position) {
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
    next_state.set(mode);

    let animator = Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(100),
        TransformTranslateKnobLens {
            start: (position.x - slider_position.x - 0.5 * knob_position.width)
                / slider_position.width,
            end: match mode {
                ModeState::Normal => 0.,
                ModeState::Notes => {
                    (slider_position.width - knob_position.width) / slider_position.width
                }
            },
        },
    ));

    commands.entity(knob).insert(animator);
}

#[derive(Debug, Copy, Clone)]
struct TransformTranslateKnobLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<FlexItemStyle> for TransformTranslateKnobLens {
    fn lerp(&mut self, target: &mut FlexItemStyle, ratio: f32) {
        let x = self.start + (self.end - self.start) * ratio;
        target.transform.translation.x = x;
    }
}
