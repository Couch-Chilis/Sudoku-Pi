use crate::{constants::*, game::*, ui::*};
use crate::{Fonts, Game, ScreenState, Settings, TransitionEvent};
use bevy::prelude::*;

const INITIAL_NUMBER_INSTRUCTION: &str =
    "Go on and fill in the blue cell.\nTap and hold to open the wheel.";
const PROCEED_NUMBER_INSTRUCTION: &str = "Great!\nLet's proceed to trying out notes next.";

const INITIAL_NOTES_INSTRUCTION: &str =
    "Now it's time to try out some notes.\nTouch any open cell to \"draw\" notes.";
const PROCEED_NOTES_INSTRUCTION: &str = "Great!\nWhichever number is selected is the one you draw.";

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum OnboardingScreenAction {
    HowToPlayNumbers,
    HowToPlayNotes,
    FinishOnboarding,
}

#[derive(Component)]
pub struct OnboardingNumberInstruction;

#[derive(Component)]
pub struct OnboardingNumberHint;

#[derive(Component)]
pub struct OnboardingNotesInstruction;

#[derive(Component)]
pub struct OnboardingNotesHint;

pub fn welcome_screen_setup(props: &Props, cb: &mut ChildBuilder) {
    let resources = &props.resources;

    fragment4(props, cb,
        column(
            available_size,
            (),
            center_text(
                "Welcome to\nSudoku Pi",
                (
                    font_bold(resources),
                    font_size(80.),
                    text_color(COLOR_MAIN_DARKER),
                ),
            ),
        ),
                column(
                    available_size,
                    (),
                    center_text(
                        "You are about to\nexperience a new way\nto play Sudoku.",
                        (
                            font_medium(resources),
                            font_size(50.),
                            text_color(COLOR_MAIN_DARKER),
                        ),
                    ),
                ),
                column(
                    available_size,
                    (),
                    center_text(
                        "But first, let us show you\nhow to play.",
                        (
                            font_medium(resources),
                            font_size(40.),
                            text_color(COLOR_MAIN_DARKER),
                        ),
                    ),
                ),
        column(
            available_size,
            padding(Sides::vertical(Val::Auto)),
            primary_button(
                OnboardingScreenAction::HowToPlayNumbers,
                button_size_onboarding(resources),
                text("Next", button_text(resources)),
            ),
        ),
    )
}

pub fn how_to_play_numbers_screen_setup(props: &Props, cb: &mut ChildBuilder) {
    let resources = &props.resources;

    fragment5(props, cb, 
        column(
            available_size,
            (),
            center_text(
                "A New Way\nto Fill In Numbers",
                (font_bold(resources), font_size(80.), text_color(COLOR_MAIN_DARKER)),
            ),
        ),

        column(
            preferred_size(Val::Percent(100.), Val::Pixel(80)),
            (),
            (
                OnboardingNumberInstruction,
                center_text(
                    INITIAL_NUMBER_INSTRUCTION,
                    (font_medium(resources), font_size(40.), text_color(COLOR_MAIN_DARKER)),
                ),
            ),
        ),

        board(props, ScreenState::HowToPlayNumbers),

        column(preferred_size(Val::Percent(100.), Val::Pixel(80)), (), (
            OnboardingNumberHint,
            center_text(
                "Noticed how numbers in range were disabled?\nThis is the wheel aid that helps avoid mistakes.",
                (font_medium(resources), font_size(36.), text_color(COLOR_MAIN_DARKER))
            )
        )),

        column(
            available_size,
            padding(Sides::vertical(Val::Auto)),
            primary_button(
                OnboardingScreenAction::HowToPlayNotes,
                button_size_onboarding(resources),
                text("Next", button_text(resources)),
            ),
        )
    )
}

pub fn how_to_play_notes_screen_setup(props: &Props, cb: &mut ChildBuilder) {
    let resources = &props.resources;

    fragment5(props, cb,
        column(available_size, (), center_text(
            "A New Way\nto Draw Notes",
            (font_bold(resources), font_size(80.), text_color(COLOR_MAIN_DARKER)),
        )),

        column(preferred_size(Val::Percent(100.), Val::Pixel(80)), (), (
            OnboardingNotesInstruction,
            center_text(
                INITIAL_NOTES_INSTRUCTION,
                (font_medium(resources), font_size(40.), text_color(COLOR_MAIN_DARKER)),
            ),
        )),

        board(props, ScreenState::HowToPlayNotes),

        column(preferred_size(Val::Percent(100.), Val::Pixel(80)), (), (
            OnboardingNotesHint,
            center_text(
                "Do you want to use the wheel to select a note?\nIt's still available if you long-press.",
                (font_medium(resources), font_size(36.), text_color(COLOR_MAIN_DARKER)),
            )
        )),

        column(available_size, padding(Sides::vertical(Val::Auto)), primary_button(
            OnboardingScreenAction::FinishOnboarding,
            button_size_onboarding(resources),
            text(
                if props.settings.onboarding_finished { "Return to Menu" } else { "Start Game" },
                button_text(resources)
            )
        )),
    )
}

pub fn onboarding_screen_button_interaction(
    mut transition_events: EventWriter<TransitionEvent>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<OnboardingScreenAction>)>,
    screen: Res<State<ScreenState>>,
) {
    let next_transition = match screen.get() {
        ScreenState::Welcome => TransitionEvent::HowToPlayNumbers,
        ScreenState::HowToPlayNumbers => TransitionEvent::HowToPlayNotes,
        ScreenState::HowToPlayNotes => TransitionEvent::FinishOnboarding,
        _ => return,
    };

    if interaction_query
        .iter()
        .any(|&interaction| interaction == Interaction::Pressed)
    {
        transition_events.send(next_transition);
    }
}

pub fn how_to_play_numbers_interaction(
    mut number_instruction_query: Query<&mut Text, With<OnboardingNumberInstruction>>,
    mut number_hint_query: Query<&mut Visibility, With<OnboardingNumberHint>>,
    mut button_query: Query<(&mut Interaction, &OnboardingScreenAction)>,
    screen: Res<State<ScreenState>>,
    selection: Res<Selection>,
    settings: Res<Settings>,
    fonts: Res<Fonts>,
) {
    if *screen.get() != ScreenState::HowToPlayNumbers {
        return;
    }

    if screen.is_changed() {
        for mut instruction_text in &mut number_instruction_query {
            instruction_text.sections[0].value = INITIAL_NUMBER_INSTRUCTION.to_owned();
            instruction_text.sections[0].style.font = fonts.medium.clone();
            instruction_text.sections[0].style.color = COLOR_MAIN_DARKER;
        }
        for mut hint_visibility in &mut number_hint_query {
            *hint_visibility = Visibility::Hidden;
        }
    } else if selection.is_changed() && selection.hint.is_none() {
        for mut instruction_text in &mut number_instruction_query {
            instruction_text.sections[0].value = PROCEED_NUMBER_INSTRUCTION.to_owned();
            instruction_text.sections[0].style.font = fonts.bold.clone();
            instruction_text.sections[0].style.color = COLOR_POP_FOCUS;
        }
        if settings.enable_wheel_aid {
            for mut hint_visibility in &mut number_hint_query {
                *hint_visibility = Visibility::Visible;
            }
        }
        for (mut button_interaction, action) in &mut button_query {
            if *action == OnboardingScreenAction::HowToPlayNotes {
                *button_interaction = Interaction::Selected;
            }
        }
    }
}

pub fn how_to_play_notes_interaction(
    mut notes_instruction_query: Query<&mut Text, With<OnboardingNotesInstruction>>,
    mut notes_hint_query: Query<&mut Visibility, With<OnboardingNotesHint>>,
    mut button_query: Query<(&mut Interaction, &OnboardingScreenAction)>,
    screen: Res<State<ScreenState>>,
    fonts: Res<Fonts>,
    game: Res<Game>,
) {
    if *screen.get() != ScreenState::HowToPlayNotes {
        return;
    }

    if screen.is_changed() {
        for mut instruction_text in &mut notes_instruction_query {
            instruction_text.sections[0].value = INITIAL_NOTES_INSTRUCTION.to_owned();
            instruction_text.sections[0].style.font = fonts.medium.clone();
            instruction_text.sections[0].style.color = COLOR_MAIN_DARKER;
        }
        for mut hint_visibility in &mut notes_hint_query {
            *hint_visibility = Visibility::Hidden;
        }
    } else if game.is_changed() && game.has_notes() {
        for mut instruction_text in &mut notes_instruction_query {
            instruction_text.sections[0].value = PROCEED_NOTES_INSTRUCTION.to_owned();
            instruction_text.sections[0].style.font = fonts.bold.clone();
            instruction_text.sections[0].style.color = COLOR_POP_FOCUS;
        }
        for mut hint_visibility in &mut notes_hint_query {
            *hint_visibility = Visibility::Visible;
        }
        for (mut button_interaction, action) in &mut button_query {
            if *action == OnboardingScreenAction::FinishOnboarding {
                *button_interaction = Interaction::Selected;
            }
        }
    }
}
