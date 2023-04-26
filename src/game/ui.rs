use crate::{constants::*, utils::*, WindowSize};
use bevy::prelude::*;

use super::OnGameScreen;

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
}

pub fn init_ui(asset_server: &AssetServer, commands: &mut Commands, window_size: &WindowSize) {
    let font = asset_server.load(MENU_FONT);
    let scale = window_size.vmin_scale;

    // Regular button styling.
    let button_style = Style {
        size: vmin_size(&window_size, 25.0, 9.0),
        margin: UiRect::all(Val::Px(1.5 * scale)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 6.5 * scale,
        color: BUTTON_TEXT,
    };

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(5. * scale),
                        top: Val::Px((window_size.height - window_size.width) / 4. - 9.),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..button_style.clone()
                },
                ..default()
            },
            UiButtonAction::BackToMain,
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Menu", text_style.clone()));
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position: UiRect {
                        right: Val::Px(5. * scale),
                        top: Val::Px((window_size.height - window_size.width) / 4. - 9.),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..button_style.clone()
                },
                ..default()
            },
            UiButtonAction::Hint,
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Hint", text_style));
        });
}
