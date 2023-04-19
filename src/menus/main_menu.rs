use super::{ButtonBuilder, LogoBundle, MenuButtonAction};
use crate::{despawn, ScreenState, WindowSize};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            main_menu_setup.in_schedule(OnEnter(ScreenState::MainMenu)),
            despawn::<OnMainMenuScreen>.in_schedule(OnExit(ScreenState::MainMenu)),
        ));
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    asset_server: Res<AssetServer>,
) {
    if window_size.vmin_scale == 0.0 {
        return; // Don't do anything until we've initialized.
    }

    // Logo.
    commands.spawn((
        LogoBundle::new(&asset_server, &window_size),
        OnMainMenuScreen,
    ));

    // Buttons.
    let button_builder = ButtonBuilder::new(&asset_server, &window_size);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(22.0 * window_size.vmin_scale),
                        left: Val::Auto,
                        right: Val::Px(7.0 * window_size.vmin_scale),
                        bottom: Val::Auto,
                    },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            use MenuButtonAction::*;
            button_builder.add_with_text_and_action(parent, "New Game", GoToNewGame);
            button_builder.add_with_text_and_action(parent, "How to Play", GoToHowToPlay);
            button_builder.add_with_text_and_action(parent, "Quit", Quit);
        });
}
