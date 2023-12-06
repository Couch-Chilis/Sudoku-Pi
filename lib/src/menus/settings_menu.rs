use super::settings_toggle::*;
use crate::{ui::*, ScreenState, Settings};
use bevy::prelude::*;

#[derive(Component)]
pub enum SettingsButtonAction {
    Back,
}

pub fn settings_screen_setup(props: &Props, cb: &mut ChildBuilder) {
    use SettingsButtonAction::*;
    use SettingsToggle::*;

    fragment3(
        leaf(available_size),
        column(
            available_size,
            padding(Sides::vertical(Val::Auto)),
            fragment4(
                settings_toggle("Wheel swipe aid", EnableWheelAid),
                settings_toggle("Selected cell highlight", SelectedCellHighlight),
                settings_toggle("Show mistakes", ShowMistakes),
                settings_toggle("Auto-fill correct notes", AutofillCorrectNotes),
            ),
        ),
        column(
            available_size,
            padding(Sides::vertical(Val::Auto)),
            secondary_button(
                Back,
                button_size_settings(&props.resources),
                text("Back", button_text(&props.resources)),
            ),
        ),
    )(props, cb);
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
