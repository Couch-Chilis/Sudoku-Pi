use crate::{constants::*, transition_events::*, ui::*, ScreenState};
use bevy::{app::AppExit, prelude::*, sprite::Anchor};

#[derive(Component)]
#[allow(dead_code)]
pub enum MainScreenButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    Quit,
}

pub fn main_menu_buttons(props: &Props, cb: &mut ChildBuilder) {
    use MainScreenButtonAction::*;

    if cfg!(not(target_os = "ios")) {
        cb.spawn_with_children(
            props,
            ternary_button(
                Quit,
                (button_size_main, button_margin),
                text("Quit", button_text),
            ),
        );
    }

    cb.spawn_with_children(
        props,
        secondary_button(
            GoToHowToPlay,
            (button_size_main, button_margin_extra_height_on_ios),
            text("How to Play", button_text),
        ),
    );

    if props.game.may_continue() {
        cb.spawn_with_children(
            props,
            secondary_button(
                GoToNewGame,
                (button_size_main, button_margin),
                text("New Game", button_text),
            ),
        );
        cb.spawn_with_children(
            props,
            selected_button(
                ContinueGame,
                (button_size_main, button_margin),
                text("Continue", button_text),
            ),
        );
    } else {
        cb.spawn_with_children(
            props,
            selected_button(
                GoToNewGame,
                (button_size_main, button_margin),
                text("New Game", button_text),
            ),
        );
    }

    cb.spawn_with_children(props, leaf(available_size));

    cb.spawn_with_children(
        props,
        row(
            (
                fixed_size(Val::Percent(50.), Val::Vmin(6.)),
                align_self(Alignment::End),
            ),
            (),
            text_with_anchor(
                "© 2023 Couch Chilis",
                Anchor::CenterRight,
                (
                    font_medium,
                    font_size(30.),
                    text_color(COLOR_BOARD_LINE_MEDIUM),
                ),
            ),
        ),
    );
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
                GoToHowToPlay => transition_events.send(TransitionEvent::LearnNumbers),
                GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
