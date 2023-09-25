use super::ButtonBuilder;
use crate::{constants::*, sudoku::*, transition_events::*, ui::*, Fonts, ScreenState};
use bevy::{app::AppExit, prelude::*, sprite::Anchor};

#[derive(Component)]
#[allow(dead_code)]
pub enum MainScreenButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    Quit,
}

pub fn spawn_main_menu_buttons(
    main_section: &mut ChildBuilder,
    buttons: &ButtonBuilder,
    fonts: &Fonts,
    game: &Game,
) {
    use MainScreenButtonAction::*;

    if cfg!(not(target_os = "ios")) {
        buttons.build_ternary_with_text_and_action(
            main_section,
            "Quit",
            MainScreenButtonAction::Quit,
        );
    }
    buttons.build_secondary_with_text_and_action_and_custom_margin(
        main_section,
        "How to Play",
        GoToHowToPlay,
        if cfg!(target_os = "ios") {
            Size::new(Val::Vmin(1.5), Val::Vmin(5.))
        } else {
            Size::all(Val::Vmin(1.5))
        },
    );
    if game.may_continue() {
        buttons.build_secondary_with_text_and_action(main_section, "New Game", GoToNewGame);
        buttons.build_selected_with_text_and_action(main_section, "Continue", ContinueGame);
    } else {
        buttons.build_selected_with_text_and_action(main_section, "New Game", GoToNewGame);
    }

    main_section.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

    main_section
        .spawn(FlexBundle::new(
            FlexItemStyle::fixed_size(Val::Percent(50.), Val::Vmin(6.))
                .with_alignment(Alignment::End),
            FlexContainerStyle::row(),
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
    mut transition_events: EventWriter<TransitionEvent>,
    interaction_query: Query<(&Interaction, &MainScreenButtonAction), Changed<Interaction>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use MainScreenButtonAction::*;
            match action {
                ContinueGame => transition_events.send(TransitionEvent::ContinueGame),
                GoToHowToPlay => transition_events.send(TransitionEvent::HowToPlayNumbers),
                GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
