use crate::{constants::*, ui::*, utils::*, Images};
use crate::{Fonts, Game, GameTimer, Highscores, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};

use super::mode_slider::build_mode_slider;

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    GoToSettings,
    Hint,
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct SettingsIcon;

#[derive(Component)]
pub struct Timer;

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    images: &Images,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    build_button_row(game_screen, |icon_row| {
        icon_row.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

        build_settings_icon(icon_row, images);
    });

    build_timer_row(game_screen, |timer_row| {
        build_timer(timer_row, fonts);
    });

    build_button_row(game_screen, |button_row| {
        let button_size = if cfg!(target_os = "ios") {
            FlexItemStyle::fixed_size(Val::Pixel(80.), Val::Pixel(35.))
        } else {
            FlexItemStyle::fixed_size(Val::Vmin(25.), Val::Vmin(10.))
        };

        let buttons = ButtonBuilder::new(fonts, button_size);
        buttons.build_with_text_and_action(button_row, "Menu", UiButtonAction::BackToMain);

        build_score(button_row, fonts);

        buttons.build_secondary_with_text_and_action(button_row, "Hint", UiButtonAction::Hint);
    });

    board_builder(game_screen);

    build_mode_slider(game_screen, meshes, materials, fonts, images);
}

fn build_settings_icon(screen: &mut ChildBuilder, images: &Images) {
    // Cog.
    screen.spawn((
        SettingsIcon,
        Interaction::None,
        UiButtonAction::GoToSettings,
        FlexItemBundle::from_style(
            FlexItemStyle::fixed_size(Val::Vmin(8.), Val::Vmin(8.))
                .with_alignment(Alignment::Start)
                .with_margin(Size::all(Val::Vmin(5.)))
                .with_transform(Transform::from_2d_scale(1. / 64., 1. / 64.)),
        ),
        SpriteBundle {
            texture: images.cog.clone(),
            ..default()
        },
    ));
}

fn build_timer_row(screen: &mut EntityCommands, child_builder: impl FnOnce(&mut ChildBuilder)) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::from_item_style(
                FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(13.))
                    .with_margin(Size::all(Val::Vmin(2.5))),
            ))
            .with_children(child_builder);
    });
}

fn build_timer(row: &mut ChildBuilder, fonts: &Fonts) {
    let width = Val::Vmin(26.0);
    let height = Val::Vmin(11.0);
    let line_height = if cfg!(target_os = "ios") {
        Val::Pixel(1.)
    } else {
        0.03 * height
    };

    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 70.,
        color: COLOR_TIMER_TEXT,
    };

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

    row.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(width, line_height)),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));

    row.spawn(FlexBundle::from_item_style(FlexItemStyle::minimum_size(
        width,
        0.9 * height,
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            FlexTextBundle::from_text(Text::from_section("0:00", text_style))
                .with_translation(0., -3.),
        ));
    });

    row.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(width, line_height)),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));
}

pub fn build_button_row(
    screen: &mut EntityCommands,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(9.))
                    .with_margin(Size::new(Val::None, Val::Vmin(4.5))),
                FlexContainerStyle::row().with_gap(Val::Auto),
            ))
            .with_children(child_builder);
    });
}

fn build_score(row: &mut ChildBuilder, fonts: &Fonts) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 58.,
        color: COLOR_SCORE_TEXT,
    };

    row.spawn(FlexBundle::from_item_style(FlexItemStyle::fixed_size(
        Val::Vmin(25.0),
        Val::Vmin(9.0),
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Score,
            FlexTextBundle::from_text(Text::from_section(format_score(0), text_style)),
        ));
    });
}

pub fn on_score_changed(
    mut score: Query<&mut Text, With<Score>>,
    mut highscores: ResMut<Highscores>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    game: Res<Game>,
) {
    if game.is_changed() {
        for mut score_text in &mut score {
            score_text.sections[0].value = format_score(game.score);
        }

        if game.is_solved() {
            highscores.add(game.score, game.elapsed_secs);
            screen_state.set(ScreenState::Highscores);
        }
    }
}

pub fn on_time_changed(mut timer: Query<&mut Text, With<Timer>>, game_timer: Res<GameTimer>) {
    if game_timer.is_changed() {
        for mut timer_text in &mut timer {
            timer_text.sections[0].value = format_time(game_timer.elapsed_secs);
        }
    }
}

fn format_score(score: u32) -> String {
    if score == 1 {
        "1 pt.".to_owned()
    } else {
        format!("{score} pts.")
    }
}

pub fn settings_icon_interaction(
    images: Res<Images>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<Image>),
        (Changed<Interaction>, With<SettingsIcon>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        *image = match *interaction {
            Interaction::Selected => images.cog_pressed.clone(),
            Interaction::Pressed | Interaction::None => images.cog.clone(),
        };
    }
}
