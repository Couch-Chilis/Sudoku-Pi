use bevy::prelude::*;

use crate::{
    menus::SettingsToggle,
    settings::Settings,
    themes::{HostTheme, Theme, DARK_THEME, LIGHT_THEME},
    Interaction,
};

const NUM_TRANSITION_FRAMES: u8 = 12;

pub struct ThemeAdapterPlugin;

impl Plugin for ThemeAdapterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_theme_setting)
            .add_systems(Update, (on_settings_change, on_theme_change))
            .init_resource::<ThemeTransitionState>();
    }
}

#[derive(Default, Resource)]
struct ThemeTransitionState {
    frames_passed: u8,
    /// Whether the currently targeted theme is dark, or `None` before the first
    /// evaluation.
    target_dark: Option<bool>,
}

fn init_theme_setting(
    mut state: ResMut<ThemeTransitionState>,
    settings: Res<Settings>,
    host_theme: Res<HostTheme>,
) {
    let state = state.bypass_change_detection();
    state.frames_passed = NUM_TRANSITION_FRAMES; // Prevent initial transition.
    state.target_dark = Some(settings.theme.is_dark(host_theme.0));
}

fn on_settings_change(
    mut theme: ResMut<Theme>,
    mut state: ResMut<ThemeTransitionState>,
    settings: Res<Settings>,
    host_theme: Res<HostTheme>,
) {
    // Re-evaluated every frame so a change in either the setting or the host
    // theme triggers a transition.
    let target_dark = settings.theme.is_dark(host_theme.0);
    if state.target_dark != Some(target_dark) {
        state.target_dark = Some(target_dark);
        state.frames_passed = 0;
    }

    if state.frames_passed < NUM_TRANSITION_FRAMES {
        let target_theme = if target_dark { DARK_THEME } else { LIGHT_THEME };

        state.frames_passed += 1;
        *theme = if state.frames_passed == NUM_TRANSITION_FRAMES {
            target_theme
        } else {
            let from_theme = if target_dark { LIGHT_THEME } else { DARK_THEME };
            let factor = state.frames_passed as f32 / NUM_TRANSITION_FRAMES as f32;

            from_theme.mix(&target_theme, factor)
        };
    }
}

fn on_theme_change(
    theme: Res<Theme>,
    mut clear_color: ResMut<ClearColor>,
    mut interaction_query: Query<&mut Interaction, Without<SettingsToggle>>,
) {
    if !theme.is_changed() || theme.is_added() {
        return;
    }

    *clear_color = ClearColor(theme.background);

    for mut interaction in &mut interaction_query {
        interaction.set_changed();
    }
}
