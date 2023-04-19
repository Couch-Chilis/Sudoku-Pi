use super::{MenuButtonAction, Secondary};
use crate::{constants::*, utils::*, WindowSize};
use bevy::prelude::*;

pub struct ButtonBuilder {
    button_style: Style,
    text_style: TextStyle,
    secondary_text_style: TextStyle,
}

impl ButtonBuilder {
    pub fn new(asset_server: &AssetServer, window_size: &WindowSize) -> Self {
        let font = asset_server.load(MENU_FONT);

        // Regular button styling.
        let button_style = Style {
            size: vmin_size(&window_size, 40.0, 9.0),
            margin: UiRect::all(Val::Px(1.5 * window_size.vmin_scale)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let text_style = TextStyle {
            font: font.clone(),
            font_size: 6.5 * window_size.vmin_scale,
            color: BUTTON_TEXT,
        };

        // Secondary button styling.
        let secondary_text_style = TextStyle {
            font,
            font_size: text_style.font_size,
            color: SECONDARY_BUTTON_TEXT,
        };

        Self {
            button_style,
            text_style,
            secondary_text_style,
        }
    }

    pub fn add_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: MenuButtonAction,
    ) {
        parent
            .spawn((
                ButtonBundle {
                    style: self.button_style.clone(),
                    ..default()
                },
                action,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(text, self.text_style.clone()));
            });
    }

    pub fn add_back_with_text(&self, parent: &mut ChildBuilder, text: &str) {
        parent
            .spawn((
                ButtonBundle {
                    style: self.button_style.clone(),
                    ..default()
                },
                MenuButtonAction::BackToMain,
                Secondary,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    text,
                    self.secondary_text_style.clone(),
                ));
            });
    }
}
