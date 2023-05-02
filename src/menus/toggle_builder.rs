use crate::{constants::*, ui::*, Fonts};
use bevy::prelude::*;

pub struct ToggleBuilder {
    button_style: FlexItemStyle,
    text_style: TextStyle,
}

impl ToggleBuilder {
    pub fn new(fonts: &Fonts) -> Self {
        let button_style = FlexItemStyle {
            flex_base: Size::new(Val::Vmin(60.), Val::Vmin(14.)),
            margin: Size::all(Val::Vmin(2.)),
            ..default()
        };

        let text_style = TextStyle {
            font: fonts.menu.clone(),
            font_size: 60.,
            color: COLOR_BUTTON_TEXT,
        };

        Self {
            button_style,
            text_style,
        }
    }

    pub fn add_with_text_and_action(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((ButtonBundle::from_style(self.button_style.clone()), action))
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
}
