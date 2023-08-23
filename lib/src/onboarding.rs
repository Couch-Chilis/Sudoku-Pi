use super::ButtonBuilder;
use crate::{
    assets::Images, constants::*, game::build_board, ui::*, Fonts, Game, ScreenState, Settings,
};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum OnboardingScreenAction {
    Next,
}

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
    use OnboardingScreenAction::*;

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
            FlexItemStyle::preferred_size(Val::Auto, Val::Percent(50.)),
            FlexContainerStyle::column(),
        ))
        .with_children(|main| match screen {
            ScreenState::HowToPlayNumbers => {} //TODO:build_board(main, fonts, game, images, settings),
            ScreenState::HowToPlayNotes => {} //TODO:build_board(main, fonts, game, images, settings),
            _ => build_welcome_text(main, fonts),
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|footer| {
            let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
            let buttons = ButtonBuilder::new(fonts, button_size);
            buttons.build_selected_with_text_and_action(
                footer,
                get_next_button_text(screen, settings),
                Next,
            );
        });
}

fn build_welcome_text(parent: &mut ChildBuilder, fonts: &Fonts) {
    parent.spawn(FlexTextBundle::from_text(
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
}

pub fn onboarding_screen_button_actions(
    query: Query<(&Interaction, &OnboardingScreenAction), Changed<Interaction>>,
    current_screen: Res<State<ScreenState>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut settings: ResMut<Settings>,
) {
    let Some((_, action)) = query.get_single().ok()
        .filter(|(&interaction, _)| interaction == Interaction::Pressed) else {
        return;
    };

    match action {
        OnboardingScreenAction::Next => screen_state.set(match current_screen.get() {
            ScreenState::Welcome => ScreenState::HowToPlayNumbers,
            ScreenState::HowToPlayNumbers => ScreenState::HowToPlayNotes,
            _ => {
                if settings.welcome_finished {
                    ScreenState::MainMenu
                } else {
                    settings.welcome_finished = true;
                    settings.save();
                    ScreenState::SelectDifficulty
                }
            }
        }),
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
        if settings.welcome_finished {
            "Back to Menu"
        } else {
            "Start Game"
        }
    } else {
        "Next"
    }
}
