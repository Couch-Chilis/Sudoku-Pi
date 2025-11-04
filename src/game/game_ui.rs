use super::board;
use super::mode_slider::mode_slider;
use crate::{constants::*, ui::*, utils::*};
use crate::{Game, GameTimer, Highscores, Images, ScreenState};
use bevy::prelude::*;

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

pub fn game_screen() -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    fragment5(
        // Row with settings icon.
        top_row(fragment(leaf(available_size), settings_icon())),
        // Timer row.
        timer(),
        // Menu and hint buttons.
        top_row(fragment3(
            selected_button(
                UiButtonAction::BackToMain,
                game_screen_top_row_button_size,
                text("Menu", button_text),
            ),
            score(),
            secondary_button(
                UiButtonAction::Hint,
                game_screen_top_row_button_size,
                text("Hint", button_text),
            ),
        )),
        // Game board.
        board(ScreenState::Game),
        // Mode slider.
        mode_slider,
    )
}

fn settings_icon() -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    |props: &Props, cb: &mut ChildSpawnerCommands| {
        cb.spawn_with_children(
            props,
            image_t(
                (
                    SettingsIcon,
                    Interaction::None,
                    UiButtonAction::GoToSettings,
                ),
                props.resources.images.cog.clone(),
                (align_self(Alignment::Start), cog_size),
            ),
        );
    }
}

fn timer() -> (impl Bundle, impl FnOnce(&Props, &mut ChildSpawnerCommands)) {
    column(
        (
            preferred_size(Val::Vmin(90.), Val::Pixel(42)),
            margin(Size::new(Val::None, Val::Pixel(15))),
        ),
        (),
        fragment4(
            leaf(available_size),
            rect(COLOR_TIMER_BORDER, game_screen_timer_line_size),
            row(
                game_screen_timer_inner_size,
                (),
                text_t(
                    Timer,
                    "0:00",
                    (
                        font_medium,
                        game_screen_timer_font_size,
                        text_color(COLOR_TIMER_TEXT),
                    ),
                ),
            ),
            rect(COLOR_TIMER_BORDER, game_screen_timer_line_size),
        ),
    )
}

fn top_row<B: Bundle>(
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildSpawnerCommands)) {
    row(
        (
            game_screen_top_row_size,
            margin(Size::new(Val::None, Val::Pixel(15))),
        ),
        gap(Val::Auto),
        child,
    )
}

fn score() -> (impl Bundle, impl FnOnce(&Props, &mut ChildSpawnerCommands)) {
    row(
        game_screen_score_size,
        (),
        text_t(
            Score,
            format_score(0),
            (
                font_medium,
                game_screen_score_font_size,
                text_color(COLOR_SCORE_TEXT),
            ),
        ),
    )
}

pub fn on_score_changed(
    mut score: Query<&mut Text2d, With<Score>>,
    mut highscores: ResMut<Highscores>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    game: Res<Game>,
) {
    if game.is_changed() {
        for mut score_text in &mut score {
            score_text.0 = format_score(game.score);
        }

        if game.is_solved() {
            highscores.add(game.score, game.elapsed_secs);
            screen_state.set(ScreenState::Highscores);
        }
    }
}

pub fn on_time_changed(mut timer: Query<&mut Text2d, With<Timer>>, game_timer: Res<GameTimer>) {
    if game_timer.is_changed() {
        for mut timer_text in &mut timer {
            timer_text.0 = format_time(game_timer.elapsed_secs);
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
        (&Interaction, &mut Sprite),
        (Changed<Interaction>, With<SettingsIcon>),
    >,
) {
    for (interaction, mut sprite) in &mut interaction_query {
        sprite.image = match interaction {
            Interaction::Selected => images.cog_pressed.handle.clone(),
            Interaction::Pressed | Interaction::None => images.cog.handle.clone(),
        };
    }
}
