/*use super::ButtonBuilder;
use crate::sudoku::Game;
use crate::ui::*;
use crate::{Fonts, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};

pub fn settings_menu_setup(settings_screen: &mut EntityCommands, fonts: &Fonts, game: &Game) {
    settings_screen.with_children(|screen| {
        // Main menu buttons.
        build_button_section(screen, |parent| {
            use SettingsButtonAction::*;
            use SettingsToggle::*;
            let buttons = ButtonBuilder::new(&fonts);
            let toggles = ToggleBuilder::new(&fonts);
            toggles.add_with_text_and_action(
                parent,
                "Highlight selection lines",
                HighlightSelectionLines,
            );
            toggles.add_with_text_and_action(parent, "Show mistakes", ShowMistakes);
            buttons.add_secondary_with_text_and_action(parent, "Back", Back);
        });
    });
}

fn build_button_section(screen: &mut ChildBuilder, child_builder: impl FnOnce(&mut ChildBuilder)) {
    screen
        .spawn((FlexBundle::new(
            FlexContainerStyle {
                padding: Size::all(Val::Vmin(5.)),
                ..default()
            },
            FlexItemStyle::available_size()
                .without_occupying_space()
                .with_transform(Transform {
                    rotation: Quat::from_rotation_z(initial_rotation),
                    translation: Vec3::new(0., -1., 1.),
                    ..default()
                }),
        ),))
        .with_children(|main_section_rotation_axis| {
            main_section_rotation_axis
                .spawn(FlexBundle::new(
                    FlexContainerStyle::default(),
                    FlexItemStyle::fixed_size(Val::Vmin(100.), Val::Vmin(100.)).with_transform(
                        Transform {
                            translation: Vec3::new(0., 2., 1.),
                            ..default()
                        },
                    ),
                ))
                .with_children(child_builder);
        });
}

#[derive(Component)]
pub enum SettingsToggle {
    HighlightSelectionLines,
    ShowMistakes,
}

#[derive(Component)]
pub enum SettingsButtonAction {
    Back,
}

// Handles screen navigation based on button actions in the main screen.
pub fn settings_toggle_actions(
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
*/
