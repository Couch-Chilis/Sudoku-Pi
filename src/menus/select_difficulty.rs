/*use super::{ButtonBuilder, LogoBundle, MenuButtonAction};
use crate::{Screen, WindowSize};
use bevy::prelude::*;

#[derive(Component)]
struct Difficulty(u8);

pub fn select_difficulty_setup(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window_size: &WindowSize,
) {
    // Logo.
    commands.spawn((LogoBundle::new(&asset_server, &window_size),));

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
            Screen,
        ))
        .with_children(|parent| {
            use MenuButtonAction::*;
            button_builder.add_with_text_and_action(parent, "Easy", StartGameAtDifficulty(1));
            button_builder.add_with_text_and_action(parent, "Medium", StartGameAtDifficulty(2));
            button_builder.add_with_text_and_action(parent, "Advanced", StartGameAtDifficulty(3));
            button_builder.add_with_text_and_action(parent, "Expert", StartGameAtDifficulty(4));
            button_builder.add_back_with_text(parent, "Cancel");
        });
}*/
