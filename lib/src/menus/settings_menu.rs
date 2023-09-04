use super::{settings_toggle::*, ButtonBuilder};
use crate::{ui::*, Fonts, Images, ScreenState, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum SettingsButtonAction {
    Back,
}

pub fn settings_screen_setup(
    settings_screen: &mut EntityCommands,
    fonts: &Fonts,
    images: &Images,
    settings: &Settings,
) {
    settings_screen.with_children(|screen| {
        spawn_settings(screen, fonts, images, settings);
    });
}

pub fn spawn_settings(
    parent: &mut ChildBuilder,
    fonts: &Fonts,
    images: &Images,
    settings: &Settings,
) {
    use SettingsButtonAction::*;
    use SettingsToggle::*;

    parent.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|parent| {
            let mut toggles = SettingsToggleBuilder::new(fonts, images);
            toggles.build_settings_toggle(parent, settings, "Wheel swipe aid", EnableWheelAid);

            toggles.build_settings_toggle(
                parent,
                settings,
                "Selected cell highlight",
                SelectedCellHighlight,
            );

            toggles.build_settings_toggle(parent, settings, "Show mistakes", ShowMistakes);

            toggles.build_settings_toggle(
                parent,
                settings,
                "Auto-fill correct notes",
                AutofillCorrectNotes,
            );
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|parent| {
            let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
            let buttons = ButtonBuilder::new(fonts, button_size);
            buttons.build_secondary_with_text_and_action(parent, "Back", Back);
        });
}

pub fn settings_screen_button_actions(
    query: Query<(&Interaction, &SettingsButtonAction), Changed<Interaction>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::Back => screen_state.set(ScreenState::Game),
            }
        }
    }
}

pub fn settings_toggle_actions(
    query: Query<(&Interaction, &SettingsToggle), Changed<Interaction>>,
    mut settings: ResMut<Settings>,
) {
    for (interaction, toggle) in &query {
        if *interaction == Interaction::Pressed {
            match toggle {
                SettingsToggle::AutofillCorrectNotes => {
                    settings.autofill_correct_notes = !settings.autofill_correct_notes;
                }
                SettingsToggle::EnableWheelAid => {
                    settings.enable_wheel_aid = !settings.enable_wheel_aid;
                }
                SettingsToggle::SelectedCellHighlight => {
                    settings.selected_cell_highlight = !settings.selected_cell_highlight;
                }
                SettingsToggle::ShowMistakes => {
                    settings.show_mistakes = !settings.show_mistakes;
                }
            }

            settings.save();
        }
    }
}

// Updates the `ToggleEnabled` component when the setting is switched.
pub fn on_setting_change(
    mut commands: Commands,
    query: Query<(Entity, &SettingsToggle, Option<&ToggleEnabled>)>,
    settings: Res<Settings>,
) {
    if settings.is_changed() {
        for (entity, settings_toggle, toggle_enabled) in &query {
            let is_enabled = settings_toggle.is_enabled(&settings);
            if is_enabled {
                if toggle_enabled.is_none() {
                    commands.entity(entity).insert(ToggleEnabled);
                }
            } else if toggle_enabled.is_some() {
                commands.entity(entity).remove::<ToggleEnabled>();
            }
        }
    }
}
