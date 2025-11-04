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

pub fn main_menu_buttons(props: &Props, spawner: &mut ChildSpawnerCommands) {
    use MainScreenButtonAction::*;

    if props.game.may_continue() {
        spawner.spawn_with_children(
            props,
            selected_button(
                ContinueGame,
                (button_size_main, button_margin),
                text("Continue", button_text),
            ),
        );
        spawner.spawn_with_children(
            props,
            secondary_button(
                GoToNewGame,
                (button_size_main, button_margin),
                text("New Game", button_text),
            ),
        );
    } else {
        spawner.spawn_with_children(
            props,
            selected_button(
                GoToNewGame,
                (button_size_main, button_margin),
                text("New Game", button_text),
            ),
        );
    }

    spawner.spawn_with_children(
        props,
        secondary_button(
            GoToHowToPlay,
            (button_size_main, button_margin_extra_height_on_ios),
            text("How to Play", button_text),
        ),
    );

    if cfg!(not(any(target_os = "android", target_os = "ios"))) {
        spawner.spawn_with_children(
            props,
            ternary_button(
                Quit,
                (button_size_main, button_margin),
                text("Quit", button_text),
            ),
        );
    }

    spawner.spawn_with_children(props, leaf(available_size));

    spawner.spawn_with_children(
        props,
        row(
            (
                fixed_size(Val::Percent(50.), Val::Vmin(6.)),
                align_self(Alignment::End),
            ),
            (),
            text(
                "© 2025 Couch Chilis",
                (
                    font_medium,
                    font_size(25.),
                    text_anchor(Anchor::CENTER_RIGHT),
                    text_color(COLOR_BOARD_LINE_MEDIUM),
                ),
            ),
        ),
    );
}

pub fn main_menu_button_actions(
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit: MessageWriter<AppExit>,
    mut transitions: MessageWriter<Transition>,
    interaction_query: Query<(&Interaction, &MainScreenButtonAction), Changed<Interaction>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            use MainScreenButtonAction::*;
            match action {
                ContinueGame => {
                    transitions.write(Transition::ContinueGame);
                }
                GoToHowToPlay => {
                    transitions.write(Transition::LearnNumbers);
                }
                GoToNewGame => {
                    screen_state.set(ScreenState::SelectDifficulty);
                }
                Quit => {
                    app_exit.write(AppExit::Success);
                }
            }
        }
    }
}
