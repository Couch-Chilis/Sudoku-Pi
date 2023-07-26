use crate::{constants::*, ui::*, utils::*, GameTimer, Images};
use crate::{Fonts, Game, Highscores, ScreenState};
use bevy::sprite::Anchor;
use bevy::{ecs::system::EntityCommands, prelude::*};

use super::Selection;

#[derive(Component)]
pub enum HighscoreButtonAction {
    Back,
    NewGame,
}

#[derive(Component)]
pub struct ScoreContainer;

pub fn highscore_screen_setup(
    highscore_screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
    images: &Images,
) {
    highscore_screen.with_children(|screen| {
        screen.spawn(FlexBundle::new(
            FlexContainerStyle::row(),
            FlexItemStyle::available_size(),
        ));

        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::row(),
                FlexItemStyle::fixed_size(Val::Percent(100.), Val::CrossPercent(102.5)),
            ))
            .with_children(|wall_section| {
                // Wall.
                wall_section
                    .spawn(FlexLeafBundle::from_style(
                        FlexItemStyle::available_size().without_occupying_space(),
                    ))
                    .with_children(|square| {
                        square.spawn(SpriteBundle {
                            texture: images.wall.clone(),
                            transform: Transform::from_2d_scale(1. / 390., 1. / 400.),
                            ..default()
                        });
                    });

                let _spacer = wall_section.spawn(FlexItemBundle::from_style(
                    FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(18.8)),
                ));

                let mut score_container = wall_section.spawn((
                    ScoreContainer,
                    FlexLeafBundle::from_style(FlexItemStyle::available_size()),
                ));
                render_scores(&mut score_container, fonts, game, highscores);
            });

        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::column().with_padding(Size::new(Val::None, Val::Auto)),
                FlexItemStyle::available_size(),
            ))
            .with_children(|button_section| {
                let button_style = FlexItemStyle::fixed_size(Val::Percent(70.), Val::Vmin(10.))
                    .with_margin(Size::all(Val::Vmin(1.5)));
                let button_builder = ButtonBuilder::new(fonts, button_style);
                button_builder.build_secondary_with_text_and_action(
                    button_section,
                    "Back to Menu",
                    HighscoreButtonAction::Back,
                );
                button_builder.build_with_text_and_action(
                    button_section,
                    "Start a new Game",
                    HighscoreButtonAction::NewGame,
                );
            });
    });
}

pub fn highscore_button_actions(
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut selection: ResMut<Selection>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    query: Query<(&Interaction, &HighscoreButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                HighscoreButtonAction::Back => screen_state.set(ScreenState::MainMenu),
                HighscoreButtonAction::NewGame => {
                    *game = Game::generate(game.difficulty).unwrap();
                    *selection = Selection::new_for_game(&game);
                    game_timer.stopwatch.reset();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}

pub fn on_highscores_changed(
    mut commands: Commands,
    mut container: Query<Entity, With<ScoreContainer>>,
    fonts: Res<Fonts>,
    game: Res<Game>,
    highscores: Res<Highscores>,
) {
    if !highscores.is_changed() {
        return;
    }

    let Ok(container) = container.get_single_mut() else {
        return;
    };

    let mut score_container = commands.entity(container);
    score_container.despawn_descendants();

    render_scores(&mut score_container, &fonts, &game, &highscores);
}

fn render_scores(
    score_container: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
) {
    let height = 1. / 7.;

    score_container.with_children(|container| {
        let mut y = 0.;
        render_left(container, &fonts.medium, y, "Score:".to_owned());
        render_right(container, &fonts.medium, y, game.score.to_string());

        y += height;
        render_left(container, &fonts.medium, y, "Time:".to_owned());
        render_right(container, &fonts.medium, y, format_time(game.elapsed_secs));

        y += height;
        render_left(container, &fonts.medium, y, "Mistakes:".to_owned());
        render_right(container, &fonts.medium, y, game.num_mistakes.to_string());

        y += height;
        render_left(container, &fonts.medium, y, "Hints:".to_owned());
        render_right(container, &fonts.medium, y, game.num_hints.to_string());

        y += 2. * height;
        render_left(container, &fonts.bold, y, "Highest score:".to_owned());
        render_right(
            container,
            &fonts.bold,
            y,
            highscores
                .best_scores
                .first()
                .unwrap_or(&game.score)
                .to_string(),
        );

        y += height;
        render_left(container, &fonts.bold, y, "Best time:".to_owned());
        render_right(
            container,
            &fonts.bold,
            y,
            format_time(*highscores.best_times.first().unwrap_or(&game.elapsed_secs)),
        );
    });
}

fn render_left(score_container: &mut ChildBuilder, font: &Handle<Font>, y: f32, text: String) {
    render_score_text(
        score_container,
        font,
        Transform {
            translation: Vec3::new(0., 0.105 - y * 0.5, 1.),
            scale: Vec3::new(0.0015, 0.0016, 1.),
            ..default()
        },
        Anchor::CenterRight,
        text,
        COLOR_MAIN_DARKER,
    );
}

fn render_right(score_container: &mut ChildBuilder, font: &Handle<Font>, y: f32, text: String) {
    render_score_text(
        score_container,
        font,
        Transform {
            translation: Vec3::new(0.1, 0.105 - y * 0.5, 1.),
            scale: Vec3::new(0.0015, 0.0016, 1.),
            ..default()
        },
        Anchor::CenterLeft,
        text,
        COLOR_POP_FOCUS,
    );
}

fn render_score_text(
    score_container: &mut ChildBuilder,
    font: &Handle<Font>,
    transform: Transform,
    text_anchor: Anchor,
    text: String,
    color: Color,
) {
    let text_style = TextStyle {
        color,
        font: font.clone(),
        font_size: 40.,
    };

    score_container.spawn(Text2dBundle {
        text: Text::from_section(text, text_style),
        text_anchor,
        transform,
        ..default()
    });
}
