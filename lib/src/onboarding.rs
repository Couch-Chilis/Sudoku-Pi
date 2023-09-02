use super::ButtonBuilder;
use crate::{constants::*, game::*, ui::*};
use crate::{Fonts, Game, Images, ScreenState, Settings, TransitionEvent};
use bevy::{ecs::system::EntityCommands, prelude::*};

const INITIAL_NUMBER_INSTRUCTION: &str =
    "Go on and fill in the hint.\nTap and hold to open the wheel.";
const PROCEED_NUMBER_INSTRUCTION: &str = "Great!\nLet's proceed to trying out notes next.";

const INITIAL_NOTES_INSTRUCTION: &str =
    "Now it's time to try out some notes.\nYou can draw the selected number in any open cell.";
const PROCEED_NOTES_INSTRUCTION: &str = "Great!\nYou're ready to start your first game!";
const PROCEED_NOTES_INSTRUCTION_ALT: &str = "Great!\nLooks like you got the hang of it!";

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum OnboardingScreenAction {
    HowToPlayNumbers,
    HowToPlayNotes,
    FinishOnboarding,
}

#[derive(Component)]
pub struct OnboardingNumberInstruction;

#[derive(Component)]
pub struct OnboardingNotesInstruction;

pub fn onboarding_screen_setup(
    welcome_screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
    screen_state: ScreenState,
) {
    welcome_screen.with_children(|screen| {
        build_onboarding_screen(screen, fonts, game, images, settings, screen_state);
    });
}

pub fn build_onboarding_screen(
    parent: &mut ChildBuilder,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
    screen: ScreenState,
) {
    parent
        .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
        .with_children(|header| {
            header.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    get_title(screen),
                    TextStyle {
                        font: fonts.bold.clone(),
                        font_size: 80.,
                        color: COLOR_MAIN_DARKER,
                    },
                )
                .with_alignment(TextAlignment::Center),
            ));
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::preferred_size(Val::Percent(100.), Val::Percent(50.)),
            FlexContainerStyle::column(),
        ))
        .with_children(|main| match screen {
            ScreenState::HowToPlayNumbers => {
                build_how_to_play_numbers(main, fonts, game, images, settings)
            }
            ScreenState::HowToPlayNotes => {
                build_how_to_play_notes(main, fonts, game, images, settings)
            }
            _ => build_welcome_text(main, fonts),
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|footer| {
            let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
            let button_text = get_next_button_text(screen, settings);
            let buttons = ButtonBuilder::new(fonts, button_size);
            let build_fn = if screen == ScreenState::Welcome {
                ButtonBuilder::build_selected_with_text_and_action
            } else {
                ButtonBuilder::build_with_text_and_action
            };
            use OnboardingScreenAction::*;
            let action = match screen {
                ScreenState::Welcome => HowToPlayNumbers,
                ScreenState::HowToPlayNumbers => HowToPlayNotes,
                ScreenState::HowToPlayNotes => FinishOnboarding,
                _ => unreachable!(),
            };
            build_fn(&buttons, footer, button_text, action);
        });
}

fn build_welcome_text(parent: &mut ChildBuilder, fonts: &Fonts) {
    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::default(),
        ))
        .with_children(|top| {
            top.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    "You are about to\nexperience a new way\nto play Sudoku.",
                    TextStyle {
                        font: fonts.medium.clone(),
                        font_size: 50.,
                        color: COLOR_MAIN_DARKER,
                    },
                )
                .with_alignment(TextAlignment::Center),
            ));
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::default(),
        ))
        .with_children(|bottom| {
            bottom.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    "But first, let us show you\nhow to play.",
                    TextStyle {
                        font: fonts.medium.clone(),
                        font_size: 40.,
                        color: COLOR_MAIN_DARKER,
                    },
                )
                .with_alignment(TextAlignment::Center),
            ));
        });
}

fn build_how_to_play_numbers(
    parent: &mut ChildBuilder,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
) {
    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80.)),
            FlexContainerStyle::default(),
        ))
        .with_children(|top| {
            top.spawn((
                OnboardingNumberInstruction,
                FlexTextBundle::from_text(
                    Text::from_section(
                        INITIAL_NUMBER_INSTRUCTION,
                        TextStyle {
                            font: fonts.medium.clone(),
                            font_size: 30.,
                            color: COLOR_MAIN_DARKER,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                ),
            ));
        });

    use ScreenState::*;
    build_board(parent, fonts, game, images, settings, HowToPlayNumbers);

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80.)),
            FlexContainerStyle::default(),
        ))
        .with_children(|bottom| {
            bottom.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    "See how numbers in range are disabled?\nThis is the wheel aid that helps avoid mistakes.",
                    TextStyle {
                        font: fonts.medium.clone(),
                        font_size: 30.,
                        color: COLOR_MAIN_DARKER,
                    },
                )
                .with_alignment(TextAlignment::Center),
            ));
        });
}

fn build_how_to_play_notes(
    parent: &mut ChildBuilder,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
) {
    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80.)),
            FlexContainerStyle::default(),
        ))
        .with_children(|top| {
            top.spawn((
                OnboardingNotesInstruction,
                FlexTextBundle::from_text(
                    Text::from_section(
                        INITIAL_NOTES_INSTRUCTION,
                        TextStyle {
                            font: fonts.medium.clone(),
                            font_size: 30.,
                            color: COLOR_MAIN_DARKER,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                ),
            ));
        });

    use ScreenState::*;
    build_board(parent, fonts, game, images, settings, HowToPlayNotes);

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80.)),
            FlexContainerStyle::default(),
        ))
        .with_children(|bottom| {
            bottom.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    "Do you want to use the wheel to create a note?\nIt's still available if you long-press.",
                    TextStyle {
                        font: fonts.medium.clone(),
                        font_size: 30.,
                        color: COLOR_MAIN_DARKER,
                    },
                )
                .with_alignment(TextAlignment::Center),
            ));
        });
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
    mut button_query: Query<(&mut Interaction, &OnboardingScreenAction)>,
    screen: Res<State<ScreenState>>,
    selection: Res<Selection>,
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
    } else if selection.is_changed() && selection.hint.is_none() {
        for mut instruction_text in &mut number_instruction_query {
            instruction_text.sections[0].value = PROCEED_NUMBER_INSTRUCTION.to_owned();
            instruction_text.sections[0].style.font = fonts.bold.clone();
            instruction_text.sections[0].style.color = COLOR_POP_FOCUS;
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
    mut button_query: Query<(&mut Interaction, &OnboardingScreenAction)>,
    screen: Res<State<ScreenState>>,
    settings: Res<Settings>,
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
    } else if game.is_changed() && game.has_notes() {
        for mut instruction_text in &mut notes_instruction_query {
            instruction_text.sections[0].value = if settings.onboarding_finished {
                PROCEED_NOTES_INSTRUCTION_ALT.to_owned()
            } else {
                PROCEED_NOTES_INSTRUCTION.to_owned()
            };
            instruction_text.sections[0].style.font = fonts.bold.clone();
            instruction_text.sections[0].style.color = COLOR_POP_FOCUS;
        }
        for (mut button_interaction, action) in &mut button_query {
            if *action == OnboardingScreenAction::FinishOnboarding {
                *button_interaction = Interaction::Selected;
            }
        }
    }
}

fn get_title(screen: ScreenState) -> &'static str {
    match screen {
        ScreenState::HowToPlayNumbers => "A New Way\nto Select Numbers",
        ScreenState::HowToPlayNotes => "A New Way\nto Draw Notes",
        _ => "Welcome to\nSudoku Pi",
    }
}

fn get_next_button_text(screen: ScreenState, settings: &Settings) -> &'static str {
    if screen == ScreenState::HowToPlayNotes {
        if settings.onboarding_finished {
            "Back to Menu"
        } else {
            "Start Game"
        }
    } else {
        "Next"
    }
}
