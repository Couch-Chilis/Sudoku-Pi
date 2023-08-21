use super::ButtonBuilder;
use crate::{constants::COLOR_MAIN_DARKER, settings::Settings, ui::*, Fonts, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum WelcomeScreenAction {
    Next,
}

pub fn welcome_screen_setup(welcome_screen: &mut EntityCommands, fonts: &Fonts, title: &str) {
    welcome_screen.with_children(|screen| {
        build_welcome_screen(screen, fonts, title);
    });
}

pub fn build_welcome_screen(parent: &mut ChildBuilder, fonts: &Fonts, title: &str) {
    use WelcomeScreenAction::*;

    parent
        .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
        .with_children(|header| {
            header.spawn(FlexTextBundle::from_text(
                Text::from_section(
                    title,
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
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|_main| {
            // TODO
        });

    parent
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::column().with_padding(Sides::vertical(Val::Auto)),
        ))
        .with_children(|footer| {
            let button_size = FlexItemStyle::fixed_size(Val::Vmin(50.), Val::Vmin(10.));
            let buttons = ButtonBuilder::new(fonts, button_size);
            buttons.build_selected_with_text_and_action(footer, "Next", Next);
        });
}

pub fn welcome_screen_button_actions(
    query: Query<(&Interaction, &WelcomeScreenAction), Changed<Interaction>>,
    current_screen: Res<State<ScreenState>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut settings: ResMut<Settings>,
) {
    let Some((_, action)) = query.get_single().ok()
        .filter(|(&interaction, _)| interaction == Interaction::Pressed) else {
        return;
    };

    match action {
        WelcomeScreenAction::Next => screen_state.set(match current_screen.get() {
            ScreenState::Welcome1 => ScreenState::Welcome2,
            ScreenState::Welcome2 => ScreenState::Welcome3,
            _ => {
                settings.welcome_finished = true;
                settings.save();
                ScreenState::MainMenu
            }
        }),
    }
}
