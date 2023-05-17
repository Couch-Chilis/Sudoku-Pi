use super::{ButtonBuilder, SettingsToggle, SettingsToggleBuilder};
use crate::{constants::*, ui::*, Fonts, ScreenState, Settings};
use bevy::prelude::*;

#[derive(Component)]
pub enum SettingsButtonAction {
    Back,
}

pub fn spawn_settings(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    settings: &Settings,
) {
    use SettingsButtonAction::*;
    use SettingsToggle::*;

    let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(11.));
    let buttons = ButtonBuilder::new(fonts, button_size);
    buttons.build_ternary_with_text_and_action(parent, "Back", Back);

    parent.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
        Val::Auto,
        Val::Vmin(8.),
    )));

    let mut toggles = SettingsToggleBuilder::new(fonts, meshes, materials);
    toggles.build_settings_toggle(
        parent,
        settings,
        "Highlight selection lines",
        HighlightSelectionLines,
    );

    toggles.build_settings_toggle(parent, settings, "Show mistakes", ShowMistakes);
}

pub fn settings_screen_button_actions(
    query: Query<(&Interaction, &SettingsButtonAction), (Changed<Interaction>, With<Button>)>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            match action {
                SettingsButtonAction::Back => screen_state.set(ScreenState::MainMenu),
            }
        }
    }
}

pub fn settings_toggle_actions(
    query: Query<(&Interaction, &SettingsToggle), Changed<Interaction>>,
    mut settings: ResMut<Settings>,
) {
    for (interaction, toggle) in &query {
        if *interaction == Interaction::JustPressed {
            match toggle {
                SettingsToggle::HighlightSelectionLines => {
                    settings.highlight_selection_lines = !settings.highlight_selection_lines;
                }
                SettingsToggle::ShowMistakes => {
                    settings.show_mistakes = !settings.show_mistakes;
                }
            }
        }
    }
}

// Updates the toggle styling when the setting is switched.
pub fn on_setting_change(
    mut query: Query<(&mut Handle<ColorMaterial>, &SettingsToggle), With<Toggle>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut material, toggle) in &mut query {
        *material = materials.add(ColorMaterial::from(if toggle.is_enabled(&settings) {
            COLOR_TOGGLE_ON
        } else {
            COLOR_TOGGLE_OFF
        }));
    }
}
