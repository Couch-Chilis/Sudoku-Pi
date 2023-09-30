use crate::{constants::*, ui::*, utils::*, Fortune, Images, ScreenSizing, TransitionEvent};
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
pub struct StatsContainer;

#[derive(Component)]
pub struct ScrollQuoteText;

#[derive(Component)]
pub struct StatTextMarker {
    kind: StatKind,
}

impl StatTextMarker {
    fn new(kind: StatKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StatKind {
    Score,
    Time,
    Mistakes,
    Hints,
    HighestScore,
    BestTime,
}

#[derive(Component)]
pub struct BestTimeText;

pub fn highscore_screen_setup(
    highscore_screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
    images: &Images,
    screen_sizing: &ScreenSizing,
) {
    highscore_screen.with_children(|screen| {
        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::all(Val::Vmin(5.))),
            ))
            .with_children(|scroll_section| {
                let item_style = if screen_sizing.is_ipad {
                    FlexItemStyle::fixed_size(Val::Percent(45.), Val::CrossPercent(17.1))
                } else {
                    FlexItemStyle::fixed_size(Val::Percent(90.), Val::CrossPercent(34.3))
                };

                // Scroll.
                scroll_section.spawn((
                    FlexItemBundle::from_style(
                        item_style
                            .clone()
                            .with_alignment(Alignment::Centered)
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
                        item_style
                            .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
                        FlexContainerStyle::column().with_padding(Sides::all(Val::Vmin(10.))),
                    ))
                    .with_children(|scroll_text_container| {
                        scroll_text_container.spawn((
                            ScrollQuoteText,
                            FlexTextBundle::from_text(Text::default()).with_bounds(Text2dBounds {
                                size: Vec2::new(550., 200.),
                            }),
                        ));
                    });
            });

        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::fixed_size(
                    Val::Percent(100.),
                    Val::CrossPercent(if screen_sizing.is_ipad { 59.8 } else { 102.5 }),
                ),
                FlexContainerStyle::row(),
            ))
            .with_children(|wall_section| {
                // Wall.
                wall_section.spawn((
                    FlexItemBundle::from_style(
                        FlexItemStyle::available_size()
                            .without_occupying_space()
                            .with_transform(if screen_sizing.is_ipad {
                                Transform::from_2d_scale(1. / 2503., 1. / 1497.)
                            } else {
                                Transform::from_2d_scale(1. / 780., 1. / 797.)
                            }),
                    ),
                    SpriteBundle {
                        texture: if screen_sizing.is_ipad {
                            images.wall_ipad.clone()
                        } else {
                            images.wall.clone()
                        },
                        ..default()
                    },
                ));

                let _spacer = wall_section.spawn(FlexItemBundle::from_style(
                    FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(18.8))
                        .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
                ));

                let padding = if screen_sizing.is_ipad {
                    Sides {
                        top: Val::Percent(35.),
                        right: Val::Percent(30.),
                        bottom: Val::Percent(15.),
                        left: Val::Percent(30.),
                    }
                } else {
                    Sides {
                        top: Val::Percent(30.),
                        right: Val::Percent(15.),
                        bottom: Val::Percent(10.),
                        left: Val::Percent(15.),
                    }
                };
                let mut score_container = wall_section.spawn((
                    StatsContainer,
                    FlexBundle::new(
                        FlexItemStyle::available_size()
                            .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
                        FlexContainerStyle::column().with_padding(padding),
                    ),
                ));
                render_scores(&mut score_container, fonts, game, highscores);
            });

        screen
            .spawn(FlexBundle::new(
                FlexItemStyle::available_size(),
                FlexContainerStyle::column().with_padding(Sides::new(Val::None, Val::Auto)),
            ))
            .with_children(|button_section| {
                let button_style = if screen_sizing.is_ipad {
                    FlexItemStyle::fixed_size(Val::Vmin(35.), Val::Vmin(5.))
                        .with_margin(Size::all(Val::Vmin(1.5)))
                } else {
                    FlexItemStyle::fixed_size(Val::Vmin(70.), Val::Vmin(10.))
                        .with_margin(Size::all(Val::Vmin(1.5)))
                };
                let buttons = ButtonBuilder::new(fonts, button_style);
                buttons.build_secondary_with_text_and_action(
                    button_section,
                    "Back to Menu",
                    HighscoreButtonAction::Back,
                );
                buttons.build_selected_with_text_and_action(
                    button_section,
                    "Start a New Game",
                    HighscoreButtonAction::NewGame,
                );
            });
    });
}

pub fn highscore_button_actions(
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut transition_events: EventWriter<TransitionEvent>,
    query: Query<(&Interaction, &HighscoreButtonAction), (Changed<Interaction>, With<Button>)>,
    game: Res<Game>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                HighscoreButtonAction::Back => screen_state.set(ScreenState::MainMenu),
                HighscoreButtonAction::NewGame => {
                    transition_events.send(TransitionEvent::StartGame(game.difficulty))
                }
            }
        }
    }
}

pub fn on_highscores_changed(
    mut stats_query: Query<(&mut Text, &StatTextMarker)>,
    game: Res<Game>,
    highscores: Res<Highscores>,
) {
    if !highscores.is_changed() {
        return;
    }

    for (mut text, marker) in &mut stats_query {
        text.sections[0].value = get_stat_text(marker.kind, &game, &highscores);
    }
}

fn get_stat_text(kind: StatKind, game: &Game, highscores: &Highscores) -> String {
    match kind {
        StatKind::Score => game.score.to_string(),
        StatKind::Time => format_time(game.elapsed_secs),
        StatKind::Mistakes => game.num_mistakes.to_string(),
        StatKind::Hints => game.num_hints.to_string(),
        StatKind::HighestScore => highscores
            .best_scores
            .first()
            .unwrap_or(&game.score)
            .to_string(),
        StatKind::BestTime => {
            format_time(*highscores.best_times.first().unwrap_or(&game.elapsed_secs))
        }
    }
}

fn render_scores(
    score_container: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
) {
    score_container.with_children(|container| {
        let mut create_row = |marker: StatTextMarker, label: &str| {
            create_stat_row(container, fonts, game, highscores, marker, label);
        };

        create_row(StatTextMarker::new(StatKind::Score), "Score:");
        create_row(StatTextMarker::new(StatKind::Time), "Time:");
        create_row(StatTextMarker::new(StatKind::Mistakes), "Mistakes:");
        create_row(StatTextMarker::new(StatKind::Hints), "Hints:");

        let _spacer = container.spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()));

        let mut create_row = |marker: StatTextMarker, label: &str| {
            create_stat_row(container, fonts, game, highscores, marker, label);
        };

        create_row(
            StatTextMarker::new(StatKind::HighestScore),
            "Highest score:",
        );
        create_row(StatTextMarker::new(StatKind::BestTime), "Best time:");
    });
}

fn create_stat_row(
    container: &mut ChildBuilder,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
    marker: StatTextMarker,
    label: &str,
) {
    let font = if matches!(marker.kind, StatKind::HighestScore | StatKind::BestTime) {
        fonts.bold.clone()
    } else {
        fonts.medium.clone()
    };

    container
        .spawn(FlexBundle::new(
            FlexItemStyle::available_size(),
            FlexContainerStyle::row(),
        ))
        .with_children(|row| {
            row.spawn(FlexBundle::from_item_style(FlexItemStyle::preferred_size(
                Val::Percent(50.),
                Val::Percent(100.),
            )))
            .with_children(|left| {
                let style = TextStyle {
                    font: font.clone(),
                    font_size: 40.,
                    color: COLOR_MAIN_DARKER,
                };

                left.spawn(
                    FlexTextBundle::from_text(Text::from_section(label, style))
                        .with_anchor(Anchor::CenterRight),
                );
            });

            row.spawn(FlexBundle::from_item_style(
                FlexItemStyle::preferred_size(Val::Percent(40.), Val::Percent(100.))
                    .with_margin(Size::new(Val::Percent(5.), Val::None)),
            ))
            .with_children(|right| {
                let value = get_stat_text(marker.kind, game, highscores);
                let style = TextStyle {
                    font,
                    font_size: 40.,
                    color: COLOR_POP_FOCUS,
                };

                right.spawn((
                    marker,
                    FlexTextBundle::from_text(Text::from_section(value, style))
                        .with_anchor(Anchor::CenterLeft),
                ));
            });
        });
}

pub fn on_fortune(
    mut scroll_quote: Query<&mut Text, With<ScrollQuoteText>>,
    fonts: Res<Fonts>,
    fortune: Res<Fortune>,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() || screen_state.get() != &ScreenState::Highscores {
        return;
    }

    let line_index = rand::random::<usize>() % fortune.lines.len();
    let line = fortune.lines[line_index];
    let (quote, author) = if let Some(emdash_index) = line.find('—') {
        let quote = line[..emdash_index].trim_end();
        let author = line[emdash_index + '—'.len_utf8()..].trim_start();
        (quote, author)
    } else {
        (line, "")
    };

    let Ok(mut quote_text) = scroll_quote.get_single_mut() else {
        return;
    };

    let quote_style = TextStyle {
        font: fonts.scroll.clone(),
        font_size: 35.,
        color: Color::BLACK,
    };

    let author_style = TextStyle {
        font: fonts.scroll.clone(),
        font_size: 30.,
        color: Color::BLACK,
    };

    *quote_text = Text::from_sections(if author.is_empty() {
        vec![TextSection::new(quote, quote_style)]
    } else {
        vec![
            TextSection::new(format!("{quote}\n"), quote_style),
            TextSection::new(format!("— {author}"), author_style),
        ]
    })
    .with_alignment(TextAlignment::Center);
}
