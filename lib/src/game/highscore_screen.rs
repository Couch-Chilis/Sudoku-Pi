use crate::{constants::*, ui::*, utils::*, Fortune, ScreenSizing, TransitionEvent};
use crate::{Fonts, Game, Highscores, ScreenState};
use bevy::prelude::*;
use bevy::sprite::Anchor;

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

pub fn highscore_screen() -> impl FnOnce(&Props, &mut ChildBuilder) {
    fragment3(
        // Scroll section.
        column(
            available_size,
            padding(Sides::all(Val::Vmin(5.))),
            fragment3(
                column(
                    (
                        align_self(Alignment::Centered),
                        highscore_scroll_size,
                        without_occupying_space,
                    ),
                    (
                        background_color(COLOR_BOARD_LINE_THIN),
                        highscore_scroll_padding,
                    ),
                    rect(COLOR_CREAM, available_size),
                ),
                column(
                    (highscore_scroll_size, without_occupying_space, z_index(3.)),
                    highscore_scroll_padding,
                    text_t(
                        ScrollText::new(ScrollTextKind::Quote),
                        "",
                        highscore_scroll_quote_text_bounds,
                    ),
                ),
                column(
                    (highscore_scroll_size, without_occupying_space, z_index(4.)),
                    highscore_scroll_author_padding,
                    column(
                        available_size,
                        (),
                        text_t(
                            ScrollText::new(ScrollTextKind::Author),
                            "",
                            text_anchor(Anchor::BottomRight),
                        ),
                    ),
                ),
            ),
        ),
        // Wall section.
        row(
            highscore_screen_wall_size,
            (),
            fragment(
                // Wall background.
                dynamic_image(wall_image, (available_size, without_occupying_space)),
                // Score board.
                column_t(
                    StatsContainer,
                    (available_size, z_index(2.)),
                    score_board_padding,
                    scores(),
                ),
            ),
        ),
        // Buttons section.
        column(
            available_size,
            padding(Sides::new(Val::None, Val::Auto)),
            fragment(
                secondary_button(
                    HighscoreButtonAction::Back,
                    (
                        highscore_screen_button_size,
                        margin(Size::all(Val::Vmin(1.5))),
                    ),
                    text("Back to Menu", button_text),
                ),
                selected_button(
                    HighscoreButtonAction::NewGame,
                    (
                        highscore_screen_button_size,
                        margin(Size::all(Val::Vmin(1.5))),
                    ),
                    text("Start a New Game", button_text),
                ),
            ),
        ),
    )
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

fn scores() -> impl FnOnce(&Props, &mut ChildBuilder) {
    fragment7(
        stat_row(StatKind::Score, "Score:"),
        stat_row(StatKind::Time, "Time:"),
        stat_row(StatKind::Mistakes, "Mistakes:"),
        stat_row(StatKind::Hints, "Hints:"),
        leaf(available_size),
        stat_row(StatKind::HighestScore, "Highest score:"),
        stat_row(StatKind::BestTime, "Best time:"),
    )
}

fn stat_row(kind: StatKind, label: &str) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder)) {
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
                text(
                    label.to_owned(),
                    (
                        button_text_size,
                        font,
                        text_anchor(Anchor::CenterRight),
                        text_color(COLOR_MAIN_DARKER),
                    ),
                ),
            ),
            row(
                (
                    preferred_size(Val::Percent(40.), Val::Percent(100.)),
                    margin(Size::new(Val::Percent(5.), Val::None)),
                ),
                (),
                text_t(
                    StatTextMarker::from(kind),
                    "",
                    (
                        button_text_size,
                        font,
                        text_anchor(Anchor::CenterLeft),
                        text_color(COLOR_POP_FOCUS),
                    ),
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
                        font_size: if screen_sizing.is_tablet() { 60. } else { 35. },
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
                        font_size: if screen_sizing.is_tablet() { 50. } else { 30. },
                        color: Color::BLACK,
                    },
                )])
                .with_alignment(TextAlignment::Right)
            }
        }
    }
}
