use super::ButtonBuilder;
use crate::{constants::*, game::*, ui::*, ScreenSizing};
use crate::{Fonts, Game, Images, ScreenState, Settings, TransitionEvent};
use bevy::{ecs::system::EntityCommands, prelude::*};

const INITIAL_NUMBER_INSTRUCTION: &str =
    "Go on and fill in the blue cell.\nTap and hold to open the wheel.";
const PROCEED_NUMBER_INSTRUCTION: &str = "Great!\nLet's proceed to trying out notes next.";

const INITIAL_NOTES_INSTRUCTION: &str =
    "Now it's time to try out some notes.\nTouch any open cell to \"draw\" notes.";
const PROCEED_NOTES_INSTRUCTION: &str = "Great!\nYou're ready to start your first game!";
const PROCEED_NOTES_INSTRUCTION_ALT: &str =
    "Great!\nWhichever number is selected is the one you draw.";

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

pub fn welcome_screen_setup(
    screen: &mut EntityCommands,
    fonts: &Fonts,
    screen_sizing: &ScreenSizing,
) {
    screen.with_children(|parent| {
        parent
            .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
            .with_children(|header| {
                header.spawn(FlexTextBundle::from_text(
                    Text::from_section(
                        "Welcome to\nSudoku Pi",
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
            .with_children(|main| {
                main.spawn(FlexBundle::new(
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

                main.spawn(FlexBundle::new(
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
            });

        parent
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
            ))
            .with_children(|footer| {
                use OnboardingScreenAction::*;
                let buttons = make_button_builder(fonts, screen_sizing);
                buttons.build_selected_with_text_and_action(footer, "Next", HowToPlayNumbers);
            });
    });
}

pub fn how_to_play_numbers_screen_setup(
    screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    screen_sizing: &ScreenSizing,
    settings: &Settings,
) {
    screen.with_children(|parent| {
        parent
            .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
            .with_children(|header| {
                header.spawn(FlexTextBundle::from_text(
                    Text::from_section(
                        "A New Way\nto Fill In Numbers",
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
                FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80)),
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
                                font_size: 40.,
                                color: COLOR_MAIN_DARKER,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                    ),
                ));
            });

        use ScreenState::*;
        build_board(parent, fonts, game, images, HowToPlayNumbers, screen_sizing, settings);

        parent
            .spawn(FlexBundle::new(
                FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80)),
                FlexContainerStyle::default(),
            ))
            .with_children(|bottom| {
                bottom.spawn((OnboardingNumberHint, FlexTextBundle::from_text(
                    Text::from_section(
                        "Noticed how numbers in range were disabled?\nThis is the wheel aid that helps avoid mistakes.",
                        TextStyle {
                            font: fonts.medium.clone(),
                            font_size: 36.,
                            color: COLOR_MAIN_DARKER,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                )));
            });

        parent
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
            ))
            .with_children(|footer| {
                use OnboardingScreenAction::*;
                let buttons = make_button_builder(fonts, screen_sizing);
                buttons.build_with_text_and_action(footer, "Next", HowToPlayNotes);
            });
    });
}

pub fn how_to_play_notes_screen_setup(
    screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    screen_sizing: &ScreenSizing,
    settings: &Settings,
) {
    screen.with_children(|parent| {
        parent
            .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
            .with_children(|header| {
                header.spawn(FlexTextBundle::from_text(
                    Text::from_section(
                        "A New Way\nto Draw Notes",
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
                FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80)),
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
                                font_size: 40.,
                                color: COLOR_MAIN_DARKER,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                    ),
                ));
            });

        use ScreenState::*;
        build_board(parent, fonts, game, images, HowToPlayNotes, screen_sizing, settings);

        parent
            .spawn(FlexBundle::new(
                FlexItemStyle::preferred_size(Val::Percent(100.), Val::Pixel(80)),
                FlexContainerStyle::default(),
            ))
            .with_children(|bottom| {
                bottom.spawn((OnboardingNotesHint, FlexTextBundle::from_text(
                    Text::from_section(
                        "Do you want to use the wheel to select a note?\nIt's still available if you long-press.",
                        TextStyle {
                            font: fonts.medium.clone(),
                            font_size: 36.,
                            color: COLOR_MAIN_DARKER,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                )));
            });

        parent
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
            ))
            .with_children(|footer| {
                use OnboardingScreenAction::*;
                let button_text = if settings.onboarding_finished {
                    "Return to Menu"
                } else {
                    "Start Game"
                };
                let buttons = make_button_builder(fonts, screen_sizing);
                buttons.build_with_text_and_action(footer, button_text, FinishOnboarding);
            });
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
        for mut hint_visibility in &mut notes_hint_query {
            *hint_visibility = Visibility::Hidden;
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

fn make_button_builder(fonts: &Fonts, screen_sizing: &ScreenSizing) -> ButtonBuilder {
    let button_size = if screen_sizing.is_ipad {
        FlexItemStyle::fixed_size(Val::Vmin(25.), Val::Vmin(5.))
    } else {
        FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.))
    };
    ButtonBuilder::new(fonts, button_size)
}
