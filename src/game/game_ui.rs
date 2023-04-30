use crate::{constants::*, ui::*, Fonts};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
}

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    fonts: &Fonts,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    // Regular button styling.
    let button_style = FlexItemStyle::fixed_size(Val::Percent(25.0), Val::Percent(100.0));

    let text_style = TextStyle {
        font: fonts.menu.clone(),
        font_size: 60.,
        color: BUTTON_TEXT,
    };

    // Top button row.
    game_screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::with_direction(FlexDirection::Row),
                FlexItemStyle::fixed_size(Val::Vmin(90.), Val::Vmin(9.))
                    .with_margin(Size::all(Val::Vmin(5.))),
            ))
            .with_children(|top_row| {
                top_row
                    .spawn((
                        ButtonBundle::with_style(button_style.clone()),
                        UiButtonAction::BackToMain,
                    ))
                    .with_children(|button| {
                        button.spawn(Text2dBundle {
                            text: Text::from_section("Menu", text_style.clone()),
                            transform: Transform {
                                scale: Vec3::new(0.004, 0.01, 1.),
                                translation: Vec3::new(0., 0., 1.),
                                ..default()
                            },
                            ..default()
                        });
                    });

                top_row.spawn(FlexLeafBundle::spacer());

                top_row
                    .spawn((
                        ButtonBundle::with_style(button_style.clone()),
                        UiButtonAction::Hint,
                    ))
                    .with_children(|button| {
                        button.spawn(Text2dBundle {
                            text: Text::from_section("Hint", text_style.clone()),
                            transform: Transform {
                                scale: Vec3::new(0.004, 0.01, 1.),
                                translation: Vec3::new(0., 0., 1.),
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
    });

    board_builder(game_screen);
}
