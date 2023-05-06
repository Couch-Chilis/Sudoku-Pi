use super::game_ui::build_button_row;
use crate::{constants::*, ui::*, utils::*};
use crate::{Fonts, Game, Highscores, ScreenState};
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub enum HighscoreButtonAction {
    Back,
}

#[derive(Component)]
pub struct ScoreContainer;

pub fn highscore_screen_setup(
    highscore_screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
) {
    // Take up the space of two button rows to match the game screen.
    highscore_screen.with_children(|screen| {
        screen.spawn(FlexLeafBundle::from_style(FlexItemStyle {
            flex_base: Size::new(Val::Vmin(90.), Val::Vmin(18.)),
            flex_grow: 1.,
            margin: Size::all(Val::Vmin(9.)),
            ..default()
        }));
    });

    build_highscores(highscore_screen, meshes, materials, fonts, game, highscores);

    // Bottom button row.
    build_button_row(highscore_screen, 1., |button_row| {
        build_button(button_row, fonts, "Back", HighscoreButtonAction::Back);
    });
}

fn build_highscores(
    screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
    game: &Game,
    highscores: &Highscores,
) {
    screen.with_children(|screen| {
        screen
            .spawn(FlexLeafBundle::from_style(
                FlexItemStyle::preferred_and_minimum_size(
                    Size::all(Val::Vmin(90.)),
                    Size::all(Val::Vmin(50.)),
                )
                .with_fixed_aspect_ratio(),
            ))
            .with_children(|leaf| {
                leaf.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
                    material: materials.add(ColorMaterial::from(COLOR_BOARD_LINE_THICK)),
                    transform: Transform::default_2d(),
                    ..default()
                });

                let mut score_container = leaf.spawn((
                    ScoreContainer,
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(0.45).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                        ..default()
                    },
                ));

                render_scores(&mut score_container, fonts, game, highscores);
            });
    });
}

fn build_button(row: &mut ChildBuilder, fonts: &Fonts, text: &str, action: HighscoreButtonAction) {
    let button_style = FlexItemStyle::available_size().with_margin(Size::all(Val::Vmin(2.)));

    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: COLOR_BUTTON_TEXT,
    };

    row.spawn((ButtonBundle::from_style(button_style), action))
        .with_children(|button| {
            button.spawn(Text2dBundle {
                text: Text::from_section(text, text_style),
                transform: Transform {
                    scale: Vec3::new(0.0015, 0.01, 1.),
                    translation: Vec3::new(0., -0.08, 1.),
                    ..default()
                },
                ..default()
            });
        });
}

pub fn highscore_button_actions(
    mut screen_state: ResMut<NextState<ScreenState>>,
    query: Query<(&Interaction, &HighscoreButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            match action {
                HighscoreButtonAction::Back => screen_state.set(ScreenState::MainMenu),
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
        {
            render_left_score(container, fonts, 0., "Score".to_owned(), Color::BLACK);

            let mut y = height;
            for score in highscores.best_scores.iter() {
                let color = if *score == game.score {
                    COLOR_MAIN_POP_DARK
                } else {
                    Color::BLACK
                };
                render_left_score(container, fonts, y, score.to_string(), color);
                y += height;
            }

            if !highscores.best_scores.contains(&game.score) {
                render_left_score(container, fonts, y, game.score.to_string(), COLOR_ORANGE);
            }
        }
        {
            render_right_score(container, fonts, 0., "Time".to_owned(), Color::BLACK);

            let mut y = height;
            for time in highscores.best_times.iter() {
                let color = if *time == game.elapsed_secs {
                    COLOR_MAIN_POP_DARK
                } else {
                    Color::BLACK
                };
                render_right_score(container, fonts, y, format_time(*time), color);
                y += height;
            }

            if !highscores.best_times.contains(&game.elapsed_secs) {
                render_right_score(
                    container,
                    fonts,
                    y,
                    format_time(game.elapsed_secs).to_string(),
                    COLOR_ORANGE,
                );
            }
        }
    });
}

fn render_left_score(
    score_container: &mut ChildBuilder,
    fonts: &Fonts,
    y: f32,
    text: String,
    color: Color,
) {
    render_score(
        score_container,
        fonts,
        Transform {
            translation: Vec3::new(-0.05, 0.2 - y * 0.5, 1.),
            scale: Vec3::new(0.002, 0.0015, 1.),
            ..default()
        },
        Anchor::CenterRight,
        text,
        color,
    );
}

fn render_right_score(
    score_container: &mut ChildBuilder,
    fonts: &Fonts,
    y: f32,
    text: String,
    color: Color,
) {
    render_score(
        score_container,
        fonts,
        Transform {
            translation: Vec3::new(0.05, 0.2 - y * 0.5, 1.),
            scale: Vec3::new(0.002, 0.0015, 1.),
            ..default()
        },
        Anchor::CenterLeft,
        text,
        color,
    );
}

fn render_score(
    score_container: &mut ChildBuilder,
    fonts: &Fonts,
    transform: Transform,
    text_anchor: Anchor,
    text: String,
    color: Color,
) {
    let text_style = TextStyle {
        color,
        font: fonts.medium.clone(),
        font_size: 40.,
    };

    score_container.spawn(Text2dBundle {
        text: Text::from_section(text, text_style),
        text_anchor,
        transform,
        ..default()
    });
}

fn _determine_num_entries(game: &Game, highscores: &Highscores) -> (usize, usize) {
    let num_scores = if highscores.best_scores.contains(&game.score) {
        highscores.best_scores.len().min(MAX_NUM_HIGHSCORES)
    } else {
        MAX_NUM_HIGHSCORES + 1
    };

    let num_times = if highscores.best_times.contains(&game.elapsed_secs) {
        highscores.best_times.len().min(MAX_NUM_HIGHSCORES)
    } else {
        MAX_NUM_HIGHSCORES + 1
    };

    (num_scores, num_times)
}
