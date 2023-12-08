use crate::{constants::*, ui::*, utils::*, Fortune, ScreenSizing, TransitionEvent};
use crate::{Fonts, Game, Highscores, ScreenState};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;

#[derive(Component)]
pub enum HighscoreButtonAction {
    Back,
    NewGame,
}

#[derive(Component)]
pub struct StatsContainer;

#[derive(Component)]
pub struct ScrollText {
    kind: ScrollTextKind,
}

impl ScrollText {
    fn new(kind: ScrollTextKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScrollTextKind {
    Quote,
    Author,
}

#[derive(Component)]
pub struct StatTextMarker {
    kind: StatKind,
}

impl From<StatKind> for StatTextMarker {
    fn from(kind: StatKind) -> Self {
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

pub fn highscore_screen(props: &Props, cb: &mut ChildBuilder) {
    let resources = &props.resources;

    cb.spawn(FlexBundle::new(
        FlexItemStyle::available_size(),
        FlexContainerStyle::column().with_padding(Sides::all(Val::Vmin(5.))),
    ))
    .with_children(|scroll_section| {
        let item_style = if resources.screen_sizing.is_ipad {
            FlexItemStyle::fixed_size(Val::Pixel(700), Val::Pixel(190))
        } else {
            FlexItemStyle::fixed_size(Val::Pixel(342), Val::Pixel(92))
        };

        let padding = if resources.screen_sizing.is_ipad {
            Sides::new(Val::Pixel(30), Val::Pixel(22))
        } else {
            Sides::new(Val::Pixel(16), Val::Pixel(10))
        };

        // "Scroll" containing the quotes.
        scroll_section
            .spawn(
                FlexBundle::new(
                    item_style
                        .clone()
                        .with_alignment(Alignment::Centered)
                        .without_occupying_space(),
                    FlexContainerStyle::default().with_padding(padding.clone()),
                )
                .with_background_color(COLOR_BOARD_LINE_THIN),
            )
            .with_children(|section| {
                section.spawn((
                    FlexItemBundle::from_style(FlexItemStyle::available_size()),
                    SpriteBundle {
                        sprite: Sprite::from_color(COLOR_CREAM),
                        ..default()
                    },
                ));
            });

        scroll_section
            .spawn(FlexBundle::new(
                item_style
                    .clone()
                    .with_transform(Transform::from_translation(Vec3::new(0., 0., 3.)))
                    .without_occupying_space(),
                FlexContainerStyle::default().with_padding(padding.clone()),
            ))
            .with_children(|scroll_text_container| {
                scroll_text_container.spawn((
                    ScrollText::new(ScrollTextKind::Quote),
                    FlexTextBundle::from_text(Text::default()).with_bounds(Text2dBounds {
                        size: Vec2::new(
                            if resources.screen_sizing.is_ipad {
                                1200.
                            } else {
                                580.
                            },
                            if resources.screen_sizing.is_ipad {
                                400.
                            } else {
                                200.
                            },
                        ),
                    }),
                ));
            });

        scroll_section
            .spawn(FlexBundle::new(
                item_style.with_transform(Transform::from_translation(Vec3::new(0., 0., 4.))),
                FlexContainerStyle::default().with_padding({
                    let top = Val::Pixel(if resources.screen_sizing.is_ipad {
                        155
                    } else {
                        65
                    });
                    let right = padding.right.clone()
                        + Val::Pixel(if resources.screen_sizing.is_ipad {
                            15
                        } else {
                            10
                        });
                    padding.with_top(top).with_right(right)
                }),
            ))
            .with_children(|scroll_author_wrapper| {
                scroll_author_wrapper
                    .spawn(FlexBundle::from_item_style(FlexItemStyle::available_size()))
                    .with_children(|scroll_author_container| {
                        scroll_author_container.spawn((
                            ScrollText::new(ScrollTextKind::Author),
                            FlexTextBundle::from_text(Text::default())
                                .with_anchor(Anchor::BottomRight),
                        ));
                    });
            });
    });

    cb.spawn_with_children(
        props,
        row(
            fixed_size(
                Val::Percent(100.),
                Val::CrossPercent(if resources.screen_sizing.is_ipad {
                    59.8
                } else {
                    102.5
                }),
            ),
            (),
            fragment(
                // Wall background.
                image(
                    if resources.screen_sizing.is_ipad {
                        resources.images.wall_ipad.clone()
                    } else {
                        resources.images.wall.clone()
                    },
                    (available_size, without_occupying_space),
                ),
                // Score board.
                column_t(
                    StatsContainer,
                    (available_size, translation(Vec3::new(0., 0., 2.))),
                    padding(if resources.screen_sizing.is_ipad {
                        Sides {
                            top: Val::Percent(32.),
                            right: Val::Percent(27.),
                            bottom: Val::Percent(12.),
                            left: Val::Percent(27.),
                        }
                    } else {
                        Sides {
                            top: Val::Percent(30.),
                            right: Val::Percent(15.),
                            bottom: Val::Percent(10.),
                            left: Val::Percent(15.),
                        }
                    }),
                    scores(props),
                ),
            ),
        ),
    );

    cb.spawn(FlexBundle::new(
        FlexItemStyle::available_size(),
        FlexContainerStyle::column().with_padding(Sides::new(Val::None, Val::Auto)),
    ))
    .with_children(|button_section| {
        let button_style = if resources.screen_sizing.is_ipad {
            FlexItemStyle::fixed_size(Val::Pixel(600), Val::Pixel(60))
                .with_margin(Size::all(Val::Vmin(1.5)))
        } else {
            FlexItemStyle::fixed_size(Val::Vmin(70.), Val::Vmin(10.))
                .with_margin(Size::all(Val::Vmin(1.5)))
        };
        let font_size = if resources.screen_sizing.is_ipad {
            66.
        } else {
            44.
        };
        let buttons = ButtonBuilder::new(resources, button_style, font_size);
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
    props_tuple: PropsTuple,
) {
    let highscores: &Res<Highscores> = &props_tuple.1;
    if !highscores.is_changed() {
        return;
    }

    for (mut text, marker) in &mut stats_query {
        text.sections[0].value = get_stat_text(&Props::from_tuple(&props_tuple), marker.kind);
    }
}

fn get_stat_text(props: &Props, kind: StatKind) -> String {
    let Props {
        game, highscores, ..
    } = props;

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

fn scores(props: &Props) -> impl FnOnce(&Props, &mut ChildBuilder) {
    fragment7(
        stat_row(props, StatKind::Score, "Score:"),
        stat_row(props, StatKind::Time, "Time:"),
        stat_row(props, StatKind::Mistakes, "Mistakes:"),
        stat_row(props, StatKind::Hints, "Hints:"),
        leaf(available_size),
        stat_row(props, StatKind::HighestScore, "Highest score:"),
        stat_row(props, StatKind::BestTime, "Best time:"),
    )
}

fn stat_row(
    props: &Props,
    kind: StatKind,
    label: &str,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder)) {
    let font = if matches!(kind, StatKind::HighestScore | StatKind::BestTime) {
        font_bold
    } else {
        font_medium
    };

    row(
        available_size,
        (),
        fragment(
            row(
                preferred_size(Val::Percent(50.), Val::Percent(100.)),
                (),
                text_with_anchor(
                    label.to_owned(),
                    Anchor::CenterRight,
                    (button_text_size, font, text_color(COLOR_MAIN_DARKER)),
                ),
            ),
            row(
                (
                    preferred_size(Val::Percent(40.), Val::Percent(100.)),
                    margin(Size::new(Val::Percent(5.), Val::None)),
                ),
                (),
                text_with_anchor_t(
                    StatTextMarker::from(kind),
                    get_stat_text(props, kind),
                    Anchor::CenterLeft,
                    (button_text_size, font, text_color(COLOR_POP_FOCUS)),
                ),
            ),
        ),
    )
}

pub fn on_fortune(
    mut scroll_text: Query<(&mut Text, &ScrollText)>,
    fonts: Res<Fonts>,
    fortune: Res<Fortune>,
    screen_sizing: Res<ScreenSizing>,
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

    for (mut text, ScrollText { kind }) in &mut scroll_text {
        match kind {
            ScrollTextKind::Quote => {
                *text = Text::from_sections([TextSection::new(
                    quote,
                    TextStyle {
                        font: fonts.scroll.clone(),
                        font_size: if screen_sizing.is_ipad { 60. } else { 35. },
                        color: Color::BLACK,
                    },
                )])
                .with_alignment(if author.is_empty() {
                    TextAlignment::Center
                } else {
                    TextAlignment::Left
                })
            }
            ScrollTextKind::Author => {
                *text = Text::from_sections([TextSection::new(
                    author,
                    TextStyle {
                        font: fonts.scroll.clone(),
                        font_size: if screen_sizing.is_ipad { 50. } else { 30. },
                        color: Color::BLACK,
                    },
                )])
                .with_alignment(TextAlignment::Right)
            }
        }
    }
}
