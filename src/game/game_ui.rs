use crate::{constants::*, ui::*};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
}

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    asset_server: &AssetServer,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    let font = asset_server.load(MENU_FONT);

    // Regular button styling.
    let button_style = FlexItemStyle {
        flex_base: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
        ..default()
    };

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.,
        color: BUTTON_TEXT,
    };

    // Top button row.
    game_screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::with_direction(FlexDirection::Row),
                FlexItemStyle {
                    flex_base: Size::new(Val::Vmin(90.), Val::Vmin(9.)),
                    margin: Size::all(Val::Vmin(5.)),
                    ..default()
                },
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
                            transform: Transform::from_scale(Vec3::new(0.004, 0.01, 1.)),
                            ..default()
                        });
                    });

                top_row.spawn(FlexItemBundle::spacer());

                top_row
                    .spawn((
                        ButtonBundle::with_style(button_style.clone()),
                        UiButtonAction::Hint,
                    ))
                    .with_children(|button| {
                        button.spawn(Text2dBundle {
                            text: Text::from_section("Hint", text_style.clone()),
                            transform: Transform::from_scale(Vec3::new(0.004, 0.01, 1.)),
                            ..default()
                        });
                    });
            });
    });

    board_builder(game_screen);
}
