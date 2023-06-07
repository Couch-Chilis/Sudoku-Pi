use super::ButtonBuilder;
use crate::{constants::*, game::Selection, sudoku::*, ui::*, Fonts, ScreenState};
use bevy::{app::AppExit, prelude::*, sprite::Anchor};

#[derive(Component)]
pub enum MainScreenButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    Quit,
}

pub fn spawn_main_menu_buttons(main_section: &mut ChildBuilder, fonts: &Fonts, game: &Game) {
    use MainScreenButtonAction::*;

    let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
    let buttons = ButtonBuilder::new(fonts, button_size);
    buttons.build_ternary_with_text_and_action(main_section, "Quit", Quit);
    buttons.build_secondary_with_text_and_action(main_section, "How to Play", GoToHowToPlay);
    if game.may_continue() {
        buttons.build_secondary_with_text_and_action(main_section, "New Game", GoToNewGame);
        buttons.build_selected_with_text_and_action(main_section, "Continue", ContinueGame);
    } else {
        buttons.build_selected_with_text_and_action(main_section, "New Game", GoToNewGame);
    }

    main_section.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

    main_section
        .spawn(FlexBundle::new(
            FlexContainerStyle::row(),
            FlexItemStyle::fixed_size(Val::Percent(50.), Val::Vmin(6.))
                .with_alignment(Alignment::End),
        ))
        .with_children(|parent| {
            parent.spawn(
                FlexTextBundle::from_text(Text::from_section(
                    "© 2023 Couch Chilis",
                    TextStyle {
                        font: fonts.medium.clone(),
                        font_size: 30.,
                        color: COLOR_BOARD_LINE_MEDIUM,
                    },
                ))
                .with_anchor(Anchor::CenterRight),
            );
        });
}

pub fn main_menu_button_actions(
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut selection: ResMut<Selection>,
    interaction_query: Query<(&Interaction, &MainScreenButtonAction), Changed<Interaction>>,
    game: Res<Game>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use MainScreenButtonAction::*;
            match action {
                ContinueGame => {
                    *selection = Selection::new_for_game(&game);
                    screen_state.set(ScreenState::Game);
                }
                GoToHowToPlay => screen_state.set(ScreenState::Highscores),
                GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
