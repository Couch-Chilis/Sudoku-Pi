use crate::{constants::*, ui::*, utils::*};
use crate::{Fonts, Game, GameTimer, Highscores, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};

use super::mode_slider::build_mode_slider;

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Timer;

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    game_screen.with_children(|screen| {
        screen.spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()));
    });

    build_timer_row(game_screen, |timer_row| {
        build_timer(timer_row, fonts);
    });

    build_button_row(game_screen, |button_row| {
        let button_size = FlexItemStyle::fixed_size(Val::Vmin(25.), Val::Vmin(10.));
        let buttons = ButtonBuilder::new(fonts, button_size);
        buttons.build_with_text_and_action(button_row, "Menu", UiButtonAction::BackToMain);

        build_score(button_row, fonts);

        buttons.build_secondary_with_text_and_action(button_row, "Hint", UiButtonAction::Hint);
    });

    board_builder(game_screen);

    build_mode_slider(game_screen, meshes, materials, fonts);

    game_screen.with_children(|screen| {
        screen.spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()));
    });
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
    let width = Val::Vmin(30.0);
    let height = Val::Vmin(12.0);

    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 80.,
        color: COLOR_TIMER_TEXT,
    };

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
        width,
        0.04 * height,
    )))
    .with_children(|top_border_leaf| {
        top_border_leaf.spawn(SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            transform: Transform::default_2d(),
            ..default()
        });
    });

    row.spawn(FlexBundle::from_item_style(FlexItemStyle::fixed_size(
        width,
        0.92 * height,
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            FlexTextBundle::from_text(Text::from_section("0:00", text_style)),
        ));
    });

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
        width,
        0.04 * height,
    )))
    .with_children(|bottom_border_leaf| {
        bottom_border_leaf.spawn(SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            transform: Transform::default_2d(),
            ..default()
        });
    });
}

pub fn build_button_row(
    screen: &mut EntityCommands,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::row().with_gap(Val::Auto),
                FlexItemStyle::preferred_size(Val::Vmin(90.), Val::Vmin(9.))
                    .with_margin(Size::all(Val::Vmin(4.5))),
            ))
            .with_children(child_builder);
    });
}

fn build_score(row: &mut ChildBuilder, fonts: &Fonts) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 70.,
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
            timer_text.sections[0].value = format_time(game_timer.stopwatch.elapsed_secs());
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
