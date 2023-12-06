use crate::{constants::*, ui::*, utils::*, Images, ResourceBag, Settings};
use bevy::{prelude::*, sprite::Anchor};

#[derive(Clone, Component, Copy)]
pub enum SettingsToggle {
    AutofillCorrectNotes,
    EnableWheelAid,
    SelectedCellHighlight,
    ShowMistakes,
}

#[derive(Default, Resource)]
pub struct SettingsToggleTimer {
    frames_passed: u8,
}

#[derive(Component)]
pub struct Toggle;

#[derive(Component)]
pub struct ToggleEnabled;

impl SettingsToggle {
    pub fn is_enabled(&self, settings: &Settings) -> bool {
        match self {
            SettingsToggle::AutofillCorrectNotes => settings.autofill_correct_notes,
            SettingsToggle::EnableWheelAid => settings.enable_wheel_aid,
            SettingsToggle::SelectedCellHighlight => settings.selected_cell_highlight,
            SettingsToggle::ShowMistakes => settings.show_mistakes,
        }
    }
}

pub fn settings_toggle(
    text: impl Into<String>,
    toggle: SettingsToggle,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder)) {
    let style = FlexItemStyle {
        flex_base: Size::new(Val::Vmin(90.), Val::Vmin(11.)),
        margin: Size::all(Val::Vmin(2.)),
        ..default()
    };

    let bundle = (
        Interaction::None,
        FlexBundle::new(
            style,
            FlexContainerStyle::row()
                .with_gap(Val::Vmin(2.))
                .with_padding(Sides::all(Val::Vmin(2.))),
        ),
        toggle,
    );

    let text: String = text.into();
    let spawn_children = move |props: &Props, cb: &mut ChildBuilder| {
        let Props {
            resources,
            settings,
            ..
        } = props;

        cb.spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
            .with_children(|label_container| {
                label_container.spawn(
                    FlexTextBundle::from_text(Text::from_section(text, text_style(resources)))
                        .with_anchor(Anchor::CenterLeft),
                );
            });

        let is_enabled = toggle.is_enabled(settings);
        let toggle_bundle = (
            Toggle,
            toggle,
            FlexItemBundle::from_style(
                FlexItemStyle::fixed_size(Val::CrossPercent(70.), Val::Percent(70.))
                    .with_alignment(Alignment::Centered)
                    .with_transform(Transform::from_2d_scale(1. / 121., 1. / 121.)),
            ),
            SpriteBundle {
                texture: if is_enabled {
                    resources.images.toggle_selected.clone()
                } else {
                    resources.images.toggle_deselected.clone()
                },
                ..default()
            },
        );

        if is_enabled {
            cb.spawn((ToggleEnabled, toggle_bundle));
        } else {
            cb.spawn(toggle_bundle);
        }
    };

    (bundle, spawn_children)
}

fn text_style(resources: &ResourceBag) -> TextStyle {
    TextStyle {
        color: COLOR_SECONDARY_BUTTON_TEXT,
        font: resources.fonts.medium.clone(),
        font_size: if resources.screen_sizing.is_ipad {
            72.
        } else {
            50.
        },
    }
}

pub fn render_settings_toggles(
    mut toggle_query: Query<(&mut Handle<Image>, Option<&ToggleEnabled>), With<Toggle>>,
    mut timer: ResMut<SettingsToggleTimer>,
    images: Res<Images>,
) {
    if timer.frames_passed == 1 {
        timer.frames_passed = 0;
    } else {
        timer.frames_passed += 1;
        return;
    }

    for (mut texture, toggle_enabled) in &mut toggle_query {
        let animation_images = get_animation_images(&images, toggle_enabled.is_some());
        if let Some(index) = animation_images
            .iter()
            .position(|&image| *texture == *image)
        {
            if index < animation_images.len() - 1 {
                let next_image = animation_images[index + 1];
                *texture = next_image.clone();
            }
        } else {
            *texture = animation_images[0].clone();
        }
    }
}

fn get_animation_images(images: &Images, is_enabled: bool) -> [&Handle<Image>; 6] {
    if is_enabled {
        [
            &images.toggle_select_1,
            &images.toggle_select_2,
            &images.toggle_select_3,
            &images.toggle_select_4,
            &images.toggle_select_5,
            &images.toggle_selected,
        ]
    } else {
        [
            &images.toggle_deselect_1,
            &images.toggle_deselect_2,
            &images.toggle_deselect_3,
            &images.toggle_deselect_4,
            &images.toggle_deselect_5,
            &images.toggle_deselected,
        ]
    }
}
