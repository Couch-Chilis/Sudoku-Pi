use super::{ButtonBuilder, DifficultyButtonAction, MainButtonAction};
use crate::sudoku::Game;
use crate::ui::*;
use crate::{Fonts, ScreenState};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_tweening::{Animator, Delay, EaseFunction, EaseMethod, Lens, Tween};
use std::f32::consts::PI;
use std::time::Duration;

pub fn main_menu_setup(
    main_screen: &mut EntityCommands,
    asset_server: &AssetServer,
    fonts: &Fonts,
    game: &Game,
) {
    main_screen.with_children(|screen| {
        // Logo.
        screen
            .spawn(FlexBundle::new(
                FlexContainerStyle::default(),
                FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(50.)),
            ))
            .with_children(|logo_section| {
                logo_section
                    .spawn(FlexLeafBundle::with_style(
                        FlexItemStyle::preferred_size(Val::Vmin(38.), Val::Vmin(80.))
                            .with_margin(Size::all(Val::Vmin(10.)))
                            .with_fixed_aspect_ratio(),
                    ))
                    .with_children(|square| {
                        square.spawn(SpriteBundle {
                            texture: asset_server.load("logo.png"),
                            transform: Transform {
                                translation: Vec3::new(0., 0., 1.),
                                scale: Vec3::new(1. / 241., 1. / 513., 1.),
                                ..default()
                            },
                            ..default()
                        });
                    });
            });

        // Main menu buttons.
        build_button_section(screen, 0., |main_section| {
            use MainButtonAction::*;
            let buttons = ButtonBuilder::new(&fonts);
            buttons.add_secondary_with_text_and_action(main_section, "Quit", Quit);
            buttons.add_with_text_and_action(main_section, "How to Play", GoToHowToPlay);
            buttons.add_with_text_and_action(main_section, "New Game", GoToNewGame);
            if game.may_continue() {
                buttons.add_with_text_and_action(main_section, "Continue", ContinueGame);
            }
        });

        // Difficulty buttons.
        build_button_section(screen, -0.5 * PI, |parent| {
            use DifficultyButtonAction::*;
            let buttons = ButtonBuilder::new(&fonts);
            buttons.add_secondary_with_text_and_action(parent, "Cancel", BackToMain);
            buttons.add_with_text_and_action(parent, "Expert", StartGameAtDifficulty(4));
            buttons.add_with_text_and_action(parent, "Advanced", StartGameAtDifficulty(3));
            buttons.add_with_text_and_action(parent, "Medium", StartGameAtDifficulty(2));
            buttons.add_with_text_and_action(parent, "Easy", StartGameAtDifficulty(1));
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
