use crate::{constants::*, ui::*, utils::*};
use crate::{Fonts, Game, GameTimer, Highscores, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum UiButtonAction {
    BackToMain,
    Hint,
    ModeNormal,
    ModeNotes,
    ModeDrawing,
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Timer;

pub fn init_game_ui(
    game_screen: &mut EntityCommands,
    fonts: &Fonts,
    board_builder: impl FnOnce(&mut EntityCommands),
) {
    // Timer row.
    build_timer_row(game_screen, 1., |timer_row| {
        build_timer(timer_row, fonts);
    });

    // Top button row.
    build_button_row(game_screen, 1., |button_row| {
        build_button(button_row, fonts, "Menu", UiButtonAction::BackToMain);
        build_score(button_row, fonts);
        build_secondary_button(button_row, fonts, "Hint", UiButtonAction::Hint);
    });

    board_builder(game_screen);

    // Bottom button row.
    build_button_row(game_screen, 2., |button_row| {
        build_button(button_row, fonts, "Normal", UiButtonAction::ModeNormal);
        build_button(button_row, fonts, "Notes", UiButtonAction::ModeNotes);
        build_button(button_row, fonts, "Draw", UiButtonAction::ModeDrawing);
    });
}

fn build_timer_row(
    screen: &mut EntityCommands,
    flex_grow: f32,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::default(),
                FlexItemStyle {
                    flex_base: Size::new(Val::Vmin(90.), Val::Vmin(13.)),
                    flex_grow,
                    margin: Size::all(Val::Vmin(2.5)),
                    ..default()
                },
            ))
            .with_children(child_builder);
    });
}

fn build_timer(row: &mut ChildBuilder, fonts: &Fonts) {
    let width = Val::Vmax(32.0);
    let height = Val::Vmax(13.0);

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

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
        width,
        0.92 * height,
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            Text2dBundle {
                text: Text::from_section("0:00", text_style),
                transform: Transform {
                    scale: Vec3::new(0.004, 0.01, 1.),
                    translation: Vec3::new(0., -0.1, 1.),
                    ..default()
                },
                ..default()
            },
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
    flex_grow: f32,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::row().with_gap(Val::Auto),
                FlexItemStyle {
                    flex_base: Size::new(Val::Vmin(90.), Val::Vmin(9.)),
                    flex_grow,
                    margin: Size::all(Val::Vmin(4.5)),
                    ..default()
                },
            ))
            .with_children(child_builder);
    });
}

fn build_button(row: &mut ChildBuilder, fonts: &Fonts, text: &str, action: UiButtonAction) {
    let button_style = FlexItemStyle::fixed_size(Val::Vmax(25.0), Val::Vmax(9.0));

    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: COLOR_BUTTON_TEXT,
    };

    row.spawn((ButtonBundle::from_style(button_style), action))
        .with_children(|button| {
            button.spawn(Text2dBundle {
                text: Text::from_section(text, text_style.clone()),
                transform: Transform {
                    scale: Vec3::new(0.004, 0.01, 1.),
                    translation: Vec3::new(0., -0.08, 1.),
                    ..default()
                },
                ..default()
            });
        });
}

fn build_secondary_button(
    row: &mut ChildBuilder,
    fonts: &Fonts,
    text: &str,
    action: UiButtonAction,
) {
    let button_style = FlexItemStyle::fixed_size(Val::Vmax(25.0), Val::Vmax(9.0));

    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: COLOR_SECONDARY_BUTTON_TEXT,
    };

    row.spawn((
        FlexBundle {
            container: FlexContainerBundle {
                style: FlexContainerStyle {
                    direction: FlexDirection::Row,
                    padding: Size::all(Val::Vmin(4.)),
                    ..default()
                },
                background: Sprite::from_color(COLOR_SECONDARY_BUTTON_BORDER),
                transform: Transform::default_2d(),
                ..default()
            },
            item: FlexItemBundle::from_style(button_style),
        },
        Button,
        ButtonType::Secondary,
        Interaction::default(),
        action,
    ))
    .with_children(|button| {
        button.spawn((
            FlexItemBundle::from_style(FlexItemStyle::available_size().without_occupying_space()),
            SpriteBundle {
                sprite: Sprite::from_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..default()
            },
        ));

        button.spawn(Text2dBundle {
            text: Text::from_section(text, text_style),
            transform: Transform {
                scale: Vec3::new(0.004, 0.01, 1.),
                translation: Vec3::new(0., -0.08, 3.),
                ..default()
            },
            ..default()
        });
    });
}

fn build_score(row: &mut ChildBuilder, fonts: &Fonts) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 70.,
        color: COLOR_SCORE_TEXT,
    };

    row.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
        Val::Vmax(25.0),
        Val::Vmax(9.0),
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Score,
            Text2dBundle {
                text: Text::from_section(format_score(0), text_style),
                transform: Transform {
                    scale: Vec3::new(0.004, 0.01, 1.),
                    translation: Vec3::new(0., -0.08, 1.),
                    ..default()
                },
                ..default()
            },
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
