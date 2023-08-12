use super::Selection;
use crate::{constants::*, ui::*, utils::*, Fortune, GameTimer, Images};
use crate::{Fonts, Game, Highscores, ScreenState};
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum HighscoreButtonAction {
    Back,
    NewGame,
}

#[derive(Component)]
pub struct ScoreContainer;

#[derive(Component)]
pub struct ScrollQuoteText;

pub fn highscore_screen_setup(
    highscore_screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
    images: &Images,
) {
    highscore_screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::row().with_padding(Sides::all(Val::Vmin(5.))),
            ))
            .with_children(|scroll_section| {
                // Scroll.
                scroll_section.spawn((
                    FlexItemBundle::from_style(
                        FlexItemStyle::fixed_size(Val::Percent(90.), Val::CrossPercent(34.3))
                            .with_fixed_aspect_ratio()
                            .without_occupying_space()
                            .with_transform(Transform::from_2d_scale(1. / 1416., 1. / 537.)),
                    ),
                    SpriteBundle {
                        texture: images.scroll.clone(),
                        ..default()
                    },
                ));

                scroll_section
                    .spawn(FlexBundle::new(
                        FlexItemStyle::available_size()
                            .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
                        FlexContainerStyle::column().with_padding(Sides::all(Val::Vmin(10.))),
                    ))
                    .with_children(|scroll_text_container| {
                        scroll_text_container.spawn((
                            ScrollQuoteText,
                            FlexTextBundle::from_text(Text::default())
                                .with_bounds(Text2dBounds {
                                    size: Vec2::new(550., 200.),
                                })
                                .with_translation(0., 4.),
                        ));
                    });
            });

        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::fixed_size(Val::Percent(100.), Val::CrossPercent(102.5)),
                FlexContainerStyle::row(),
            ))
            .with_children(|wall_section| {
                // Wall.
                wall_section.spawn((
                    FlexItemBundle::from_style(
                        FlexItemStyle::available_size()
                            .without_occupying_space()
                            .with_transform(Transform::from_2d_scale(1. / 780., 1. / 797.)),
                    ),
                    SpriteBundle {
                        texture: images.wall.clone(),
                        ..default()
                    },
                ));

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
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::new(Val::None, Val::Auto)),
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
                button_builder.build_selected_with_text_and_action(
                    button_section,
                    "Start a New Game",
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
                    game_timer.elapsed_secs = 0.;
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

pub fn on_fortune(
    mut scroll_quote: Query<(&mut Text, With<ScrollQuoteText>)>,
    fonts: Res<Fonts>,
    fortune: Res<Fortune>,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() || screen_state.get() != &ScreenState::Highscores {
        return;
    }

    let line_index = rand::random::<usize>() % fortune.lines.len();
    let line = fortune.lines[line_index];

    let Ok((mut quote_text, _)) = scroll_quote.get_single_mut() else {
        return;
    };

    *quote_text = Text::from_section(
        line,
        TextStyle {
            font: fonts.scroll.clone(),
            font_size: 40.,
            color: Color::BLACK,
        },
    )
    .with_alignment(TextAlignment::Center);
}
