use super::{ButtonBuilder, SettingsToggle, ToggleBuilder};
use crate::settings::Settings;
use crate::{constants::*, sudoku::*, ui::*, utils::*};
use crate::{Fonts, GameTimer, ScreenState};
use bevy::app::AppExit;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_tweening::{Animator, Delay, EaseFunction, EaseMethod, Lens, Tween};
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Component)]
pub struct SettingsIcon;

pub fn main_menu_setup(
    main_screen: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &AssetServer,
    settings: &Settings,
    fonts: &Fonts,
    game: &Game,
) {
    main_screen.with_children(|screen| {
        // Logo.
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::row().with_gap(Val::Auto),
                FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(50.)),
            ))
            .with_children(|logo_section| {
                // Workaround to keep the logo centered.
                logo_section.spawn(FlexLeafBundle::from_style(
                    FlexItemStyle::fixed_size(Val::Vmin(10.), Val::Vmin(10.))
                        .with_margin(Size::all(Val::Vmin(5.))),
                ));

                // Logo.
                logo_section
                    .spawn(FlexLeafBundle::from_style(
                        FlexItemStyle::preferred_size(Val::Vmin(38.), Val::Vmin(80.))
                            .with_margin(Size::all(Val::Vmin(10.)))
                            .with_fixed_aspect_ratio(),
                    ))
                    .with_children(|square| {
                        square.spawn(SpriteBundle {
                            texture: asset_server.load("logo.png"),
                            transform: Transform::from_2d_scale(1. / 241., 1. / 513.),
                            ..default()
                        });
                    });

                // Cog.
                logo_section.spawn((
                    SettingsIcon,
                    Interaction::None,
                    MenuButtonAction::Settings,
                    FlexItemBundle::from_style(
                        FlexItemStyle::fixed_size(Val::Vmin(10.), Val::Vmin(10.))
                            .with_alignment(Alignment::Start)
                            .with_margin(Size::all(Val::Vmin(5.)))
                            .with_transform(Transform::from_2d_scale(1. / 64., 1. / 64.)),
                    ),
                    SpriteBundle {
                        texture: asset_server.load("cog.png"),
                        ..default()
                    },
                ));
            });

        // Main menu buttons.
        build_button_section(screen, 0., |main_section| {
            use MainScreenButtonAction::*;
            let buttons = ButtonBuilder::new(fonts);
            buttons.add_ternary_with_text_and_action(main_section, "Quit", Quit);
            buttons.add_secondary_with_text_and_action(main_section, "How to Play", GoToHowToPlay);
            if game.may_continue() {
                buttons.add_secondary_with_text_and_action(main_section, "New Game", GoToNewGame);
                buttons.add_with_text_and_action(main_section, "Continue", ContinueGame);
            } else {
                buttons.add_with_text_and_action(main_section, "New Game", GoToNewGame);
            }
        });

        // Difficulty buttons.
        build_button_section(screen, -0.5 * PI, |parent| {
            use Difficulty::*;
            use DifficultyScreenButtonAction::*;
            let buttons = ButtonBuilder::new(fonts);
            buttons.add_ternary_with_text_and_action(parent, "Back", BackToMain);
            buttons.add_with_text_and_action(parent, "Easy", StartGameAtDifficulty(Easy));
            buttons.add_with_text_and_action(parent, "Medium", StartGameAtDifficulty(Medium));
            buttons.add_with_text_and_action(parent, "Advanced", StartGameAtDifficulty(Advanced));
            buttons.add_with_text_and_action(parent, "Expert", StartGameAtDifficulty(Expert));
        });

        // Settings buttons and toggles.
        build_button_section(screen, 0.5 * PI, |parent| {
            use SettingsButtonAction::*;
            use SettingsToggle::*;
            let buttons = ButtonBuilder::new(fonts);
            let mut toggles = ToggleBuilder::new(fonts, meshes, materials);
            buttons.add_ternary_with_text_and_action(parent, "Back", Back);
            parent.spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
                Val::Auto,
                Val::Vmin(8.),
            )));
            toggles.add_with_text_and_action(
                parent,
                settings,
                "Highlight selection lines",
                HighlightSelectionLines,
            );
            toggles.add_with_text_and_action(parent, settings, "Show mistakes", ShowMistakes);
        });
    });
}

fn build_button_section(
    screen: &mut ChildBuilder,
    initial_rotation: f32,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen
        .spawn((
            ButtonSection {
                initial_rotation,
                current_rotation: initial_rotation,
            },
            FlexBundle::new(
                FlexContainerStyle {
                    padding: Size::all(Val::Vmin(5.)),
                    ..default()
                },
                FlexItemStyle::available_size()
                    .without_occupying_space()
                    .with_transform(Transform {
                        rotation: Quat::from_rotation_z(initial_rotation),
                        translation: Vec3::new(0., -1., 1.),
                        ..default()
                    }),
            ),
        ))
        .with_children(|main_section_rotation_axis| {
            main_section_rotation_axis
                .spawn(FlexBundle::new(
                    FlexContainerStyle::default(),
                    FlexItemStyle::fixed_size(Val::Vmin(100.), Val::Vmin(100.)).with_transform(
                        Transform {
                            translation: Vec3::new(0., 2., 1.),
                            ..default()
                        },
                    ),
                ))
                .with_children(child_builder);
        });
}

#[derive(Component, Default)]
pub struct ButtonSection {
    initial_rotation: f32,
    current_rotation: f32,
}

#[derive(Component)]
pub enum MainScreenButtonAction {
    ContinueGame,
    GoToHowToPlay,
    GoToNewGame,
    Quit,
}

// Handles screen navigation based on button actions in the main screen.
pub fn main_screen_button_actions(
    query: Query<(&Interaction, &MainScreenButtonAction), Changed<Interaction>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            use MainScreenButtonAction::*;
            match action {
                ContinueGame => screen_state.set(ScreenState::Game),
                GoToHowToPlay => screen_state.set(ScreenState::Highscores),
                GoToNewGame => screen_state.set(ScreenState::SelectDifficulty),
                Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

#[derive(Component)]
pub enum DifficultyScreenButtonAction {
    BackToMain,
    StartGameAtDifficulty(Difficulty),
}

// Handles screen navigation based on button actions in the difficulty screen.
pub fn difficulty_screen_button_actions(
    query: Query<
        (&Interaction, &DifficultyScreenButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game: ResMut<Game>,
    mut game_timer: ResMut<GameTimer>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            use DifficultyScreenButtonAction::*;
            match action {
                BackToMain => screen_state.set(ScreenState::MainMenu),
                StartGameAtDifficulty(difficulty) => {
                    *game = Game::generate(*difficulty).unwrap();
                    game_timer.stopwatch.reset();
                    screen_state.set(ScreenState::Game);
                }
            }
        }
    }
}

#[derive(Component)]
pub enum SettingsButtonAction {
    Back,
}

// Handles screen navigation based on button actions in the settings screen.
pub fn settings_screen_button_actions(
    query: Query<(&Interaction, &SettingsButtonAction), (Changed<Interaction>, With<Button>)>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            match action {
                SettingsButtonAction::Back => screen_state.set(ScreenState::MainMenu),
            }
        }
    }
}

// Handles toggling of settings.
pub fn settings_toggle_actions(
    query: Query<(&Interaction, &SettingsToggle), Changed<Interaction>>,
    mut settings: ResMut<Settings>,
) {
    for (interaction, toggle) in &query {
        if *interaction == Interaction::JustPressed {
            match toggle {
                SettingsToggle::HighlightSelectionLines => {
                    settings.highlight_selection_lines = !settings.highlight_selection_lines;
                }
                SettingsToggle::ShowMistakes => {
                    settings.show_mistakes = !settings.show_mistakes;
                }
            }
        }
    }
}

// Updates the toggle styling when the setting is switched.
pub fn on_setting_change(
    mut query: Query<(&mut Handle<ColorMaterial>, &SettingsToggle), With<Toggle>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut material, toggle) in &mut query {
        *material = materials.add(ColorMaterial::from(if toggle.is_enabled(&settings) {
            COLOR_TOGGLE_ON
        } else {
            COLOR_TOGGLE_OFF
        }));
    }
}

pub fn on_screen_change(
    mut commands: Commands,
    screen_state: Res<State<ScreenState>>,
    mut button_sections: Query<(Entity, &mut ButtonSection)>,
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    let new_rotation = match screen_state.0 {
        ScreenState::MainMenu | ScreenState::Game => 0.,
        ScreenState::SelectDifficulty => 0.5 * PI,
        ScreenState::Settings => -0.5 * PI,
        _ => return,
    };

    for (entity, mut button_section) in &mut button_sections {
        let start = button_section.current_rotation;
        let end = button_section.initial_rotation + new_rotation;
        if start == end {
            continue;
        }

        let animator = match screen_state.0 {
            // When going from the difficulty selection to the game, we just
            // reset the transform without animation so everything is back to
            // the starting when position when going out of the game.
            ScreenState::Game => {
                Animator::new(Delay::new(Duration::from_millis(200)).then(Tween::new(
                    EaseMethod::Discrete(0.),
                    Duration::from_millis(1),
                    TransformRotationZLens { start, end },
                )))
            }
            _ => Animator::new(Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(200),
                TransformRotationZLens { start, end },
            )),
        };

        commands.entity(entity).insert(animator);

        button_section.current_rotation = end;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TransformRotationZLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Transform> for TransformRotationZLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.rotation = Quat::from_rotation_z(value);
    }
}

pub fn menu_interaction(
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<Image>),
        (Changed<Interaction>, With<SettingsIcon>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        *image = match *interaction {
            Interaction::JustPressed | Interaction::Pressed => asset_server.load("cog_pressed.png"),
            Interaction::None | Interaction::Hovered => asset_server.load("cog.png"),
        };
    }
}

#[derive(Component)]
pub enum MenuButtonAction {
    Settings,
}

// Handles screen navigation based on button actions that persist across menus.
pub fn menu_button_actions(
    query: Query<(&Interaction, &MenuButtonAction), Changed<Interaction>>,
    current_state: Res<State<ScreenState>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::JustPressed {
            use MenuButtonAction::*;
            match action {
                Settings => screen_state.set(if current_state.0 == ScreenState::Settings {
                    ScreenState::MainMenu
                } else {
                    ScreenState::Settings
                }),
            }
        }
    }
}
