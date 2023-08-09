use crate::{constants::*, ui::*, Fonts, Settings};
use bevy::{prelude::*, sprite::Anchor};

#[derive(Clone, Component, Copy)]
pub enum SettingsToggle {
    AllowInvalidWheelNumbers,
    HighlightSelectionLines,
    ShowMistakes,
}

impl SettingsToggle {
    pub fn is_enabled(&self, settings: &Settings) -> bool {
        match self {
            SettingsToggle::AllowInvalidWheelNumbers => settings.allow_invalid_wheel_numbers,
            SettingsToggle::HighlightSelectionLines => settings.highlight_selection_lines,
            SettingsToggle::ShowMistakes => settings.show_mistakes,
        }
    }
}

pub struct SettingsToggleBuilder<'a> {
    container_style: FlexItemStyle,
    text_style: TextStyle,
    toggle_builder: ToggleBuilder<'a>,
}

impl<'a> SettingsToggleBuilder<'a> {
    pub fn new(
        fonts: &Fonts,
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<ColorMaterial>,
    ) -> Self {
        let container_style = FlexItemStyle {
            flex_base: Size::new(Val::Vmin(90.), Val::Vmin(11.)),
            margin: Size::all(Val::Vmin(2.)),
            ..default()
        };

        let text_style = TextStyle {
            font: fonts.medium.clone(),
            font_size: 60.,
            color: COLOR_SECONDARY_BUTTON_TEXT,
        };

        Self {
            container_style,
            text_style,
            toggle_builder: ToggleBuilder::new(meshes, materials),
        }
    }

    pub fn build_settings_toggle(
        &mut self,
        parent: &mut ChildBuilder,
        settings: &Settings,
        text: &str,
        toggle: SettingsToggle,
    ) {
        parent
            .spawn((
                ToggleContainer,
                Interaction::None,
                FlexBundle::new(
                    self.container_style.clone(),
                    FlexContainerStyle::row()
                        .with_gap(Val::Vmin(2.))
                        .with_padding(Sides::all(Val::Vmin(2.))),
                ),
                toggle,
            ))
            .with_children(|toggle_container| {
                self.toggle_builder.build_with_marker(
                    toggle_container,
                    toggle,
                    toggle.is_enabled(settings),
                );

                toggle_container
                    .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
                    .with_children(|label_container| {
                        label_container.spawn(
                            FlexTextBundle::from_text(Text::from_section(
                                text,
                                self.text_style.clone(),
                            ))
                            .with_anchor(Anchor::CenterLeft),
                        );
                    });
            });
    }
}
