use crate::resource_bag::ResourceBag;
use crate::{constants::*, sudoku::*, transition_events::*, ui::*, ScreenState};
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
    game: &Game,
    resources: &ResourceBag,
) {
    use MainScreenButtonAction::*;

    if cfg!(not(target_os = "ios")) {
        main_section.spawn_with_children(ternary_button(
            Quit,
            (button_size_main(resources), button_margin),
            text("Quit", button_text(resources)),
        ));
    }

    main_section.spawn_with_children(secondary_button(
        GoToHowToPlay,
        (
            button_size_main(resources),
            button_margin_extra_height_on_ios,
        ),
        text("How to Play", button_text(resources)),
    ));

    if game.may_continue() {
        main_section.spawn_with_children(secondary_button(
            GoToNewGame,
            (button_size_main(resources), button_margin),
            text("New Game", button_text(resources)),
        ));
        main_section.spawn_with_children(selected_button(
            ContinueGame,
            (button_size_main(resources), button_margin),
            text("Continue", button_text(resources)),
        ));
    } else {
        main_section.spawn_with_children(selected_button(
            GoToNewGame,
            (button_size_main(resources), button_margin),
            text("New Game", button_text(resources)),
        ));
    }

    main_section.spawn(leaf(available_size));

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
                        font: resources.fonts.medium.clone(),
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
