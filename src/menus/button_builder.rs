use crate::{constants::*, ui::*, Fonts};
use bevy::prelude::*;

pub struct ButtonBuilder {
    button_style: FlexItemStyle,
    text_style: TextStyle,
    secondary_text_style: TextStyle,
}

impl ButtonBuilder {
    pub fn new(fonts: &Fonts) -> Self {
        // Regular button styling.
        let button_style = FlexItemStyle {
            flex_base: Size::new(Val::Vmin(60.), Val::Vmin(14.)),
            margin: Size::all(Val::Vmin(2.)),
            ..default()
        };

        let text_style = TextStyle {
            font: fonts.menu.clone(),
            font_size: 60.,
            color: BUTTON_TEXT,
        };

        // Secondary button styling.
        let secondary_text_style = TextStyle {
            font: fonts.menu.clone(),
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
        action: impl Component,
    ) {
        parent
            .spawn((ButtonBundle::with_style(self.button_style.clone()), action))
            .with_children(|button| {
                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.text_style.clone()),
                    transform: Transform {
                        scale: Vec3::new(0.002, 0.01, 1.),
                        translation: Vec3::new(0., 0., 1.),
                        ..default()
                    },
                    ..default()
                });
            });
    }

    pub fn add_secondary_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                ButtonBundle::with_style(self.button_style.clone()),
                Secondary,
                action,
            ))
            .with_children(|button| {
                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.secondary_text_style.clone()),
                    transform: Transform {
                        scale: Vec3::new(0.002, 0.01, 1.),
                        translation: Vec3::new(0., 0., 1.),
                        ..default()
                    },
                    ..default()
                });
            });
    }
}
