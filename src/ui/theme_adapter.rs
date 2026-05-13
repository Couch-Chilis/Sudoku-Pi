use bevy::prelude::*;

use crate::{
    menus::SettingsToggle,
    settings::{Settings, ThemeSetting},
    themes::{Theme, DARK_THEME, LIGHT_THEME},
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
    theme_setting: ThemeSetting,
}

fn init_theme_setting(mut state: ResMut<ThemeTransitionState>, settings: Res<Settings>) {
    let state = state.bypass_change_detection();
    state.frames_passed = NUM_TRANSITION_FRAMES; // Prevent initial transition.
    state.theme_setting = settings.theme;
}

fn on_settings_change(
    mut theme: ResMut<Theme>,
    mut state: ResMut<ThemeTransitionState>,
    settings: Res<Settings>,
) {
    if settings.theme != state.theme_setting {
        state.theme_setting = settings.theme;
        state.frames_passed = 0;
    }

    if state.frames_passed < NUM_TRANSITION_FRAMES {
        let target_theme = match state.theme_setting {
            ThemeSetting::Light => LIGHT_THEME,
            ThemeSetting::Dark => DARK_THEME,
        };

        state.frames_passed += 1;
        *theme = if state.frames_passed == NUM_TRANSITION_FRAMES {
            target_theme
        } else {
            let from_theme = match state.theme_setting {
                ThemeSetting::Light => DARK_THEME,
                ThemeSetting::Dark => LIGHT_THEME,
            };
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
