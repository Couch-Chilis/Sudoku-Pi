use crate::{constants::*, ui::*, Fonts};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
    ModeNormal,
    ModeNotes,
    ModeDrawing,
}

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    fonts: &Fonts,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    // Top button row.
    build_button_row(game_screen, |top_row| {
        build_button(top_row, fonts, "Menu", UiButtonAction::BackToMain);
        top_row.spawn(FlexLeafBundle::spacer());
        build_button(top_row, fonts, "Hint", UiButtonAction::Hint);
    });

    board_builder(game_screen);

    // Bottom button row.
    build_button_row(game_screen, |bottom_row| {
        build_button(bottom_row, fonts, "Normal", UiButtonAction::ModeNormal);
        build_button(bottom_row, fonts, "Notes", UiButtonAction::ModeNotes);
        build_button(bottom_row, fonts, "Draw", UiButtonAction::ModeDrawing);
    });
}

fn build_button_row(screen: &mut EntityCommands, child_builder: impl FnOnce(&mut ChildBuilder)) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::with_direction(FlexDirection::Row),
                FlexItemStyle::minimum_size(Val::Vmin(90.), Val::Vmin(9.))
                    .with_margin(Size::all(Val::Vmin(5.))),
            ))
            .with_children(child_builder);
    });
}

fn build_button(row: &mut ChildBuilder, fonts: &Fonts, text: &str, action: UiButtonAction) {
    let button_style = FlexItemStyle::fixed_size(Val::Vmax(25.0), Val::Vmax(9.0));

    let text_style = TextStyle {
        font: fonts.menu.clone(),
        font_size: 60.,
        color: BUTTON_TEXT,
    };

    row.spawn((ButtonBundle::with_style(button_style), action))
        .with_children(|button| {
            button.spawn(Text2dBundle {
                text: Text::from_section(text, text_style.clone()),
                transform: Transform {
                    scale: Vec3::new(0.004, 0.01, 1.),
                    translation: Vec3::new(0., 0., 1.),
                    ..default()
                },
                ..default()
            });
        });
}
