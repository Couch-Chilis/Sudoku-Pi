use super::{build_logo, ButtonBuilder, MenuButtonAction};
use crate::sudoku::Game;
use crate::ui::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

pub fn main_menu_setup(main_screen: &mut EntityCommands, asset_server: &AssetServer, game: &Game) {
    main_screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle {
                    direction: FlexDirection::Column,
                    padding: Size::all(Val::Vmin(5.)),
                },
                FlexItemStyle::maximum_size(),
            ))
            .with_children(|column| {
                // Logo
                build_logo(column, asset_server);

                // Buttons.
                use MenuButtonAction::*;
                let button_builder = ButtonBuilder::new(asset_server);
                button_builder.add_secondary_with_text_and_action(column, "Quit", Quit);
                button_builder.add_with_text_and_action(column, "How to Play", GoToHowToPlay);
                button_builder.add_with_text_and_action(column, "New Game", GoToNewGame);
                if game.may_continue() {
                    button_builder.add_with_text_and_action(column, "Continue", ContinueGame);
                }
            });
    });
}
