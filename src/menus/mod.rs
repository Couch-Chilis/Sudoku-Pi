mod button_builder;
mod difficulty_menu;
mod how_to_play;
mod main_menu;
mod score;
mod settings_menu;
mod settings_toggle;
mod toggle_builder;

use crate::menus::main_menu::spawn_main_menu_buttons;
use crate::{sudoku::*, ui::*, utils::*};
use crate::{Fonts, ScreenState, Settings};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseFunction, EaseMethod, Lens, Tween};
use button_builder::ButtonBuilder;
use difficulty_menu::{difficulty_screen_button_actions, spawn_difficulty_menu_buttons};
use main_menu::main_menu_button_actions;
use settings_menu::{on_setting_change, settings_screen_button_actions, settings_toggle_actions};
use settings_toggle::SettingsToggle;
use std::f32::consts::PI;
use std::time::Duration;
use toggle_builder::ToggleBuilder;

use self::settings_menu::spawn_settings;

#[derive(Component)]
enum MenuButtonAction {
    Settings,
}

#[derive(Component, Default)]
struct ButtonSection {
    initial_rotation: f32,
    current_rotation: f32,
}

#[derive(Component)]
struct MainButtonContainer;

#[derive(Component)]
struct DifficultyButtonContainer;

#[derive(Component)]
struct SettingsButtonContainer;

#[derive(Component)]
struct SettingsIcon;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            difficulty_screen_button_actions.run_if(in_state(ScreenState::SelectDifficulty)),
        )
        .add_system(main_menu_button_actions.run_if(in_state(ScreenState::MainMenu)))
        .add_system(settings_screen_button_actions.run_if(in_state(ScreenState::Settings)))
        .add_system(settings_toggle_actions.run_if(in_state(ScreenState::Settings)))
        .add_system(menu_interaction.run_if(in_main_menu))
        .add_system(menu_button_actions.run_if(in_main_menu))
        .add_system(on_setting_change)
        .add_system(on_screen_change);
    }
}

fn in_main_menu(state: Res<State<ScreenState>>) -> bool {
    matches!(
        state.0,
        ScreenState::MainMenu | ScreenState::SelectDifficulty | ScreenState::Settings
    )
}

pub fn menu_setup(
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
        build_button_section(screen, MainButtonContainer, 0., |main_section| {
            spawn_main_menu_buttons(main_section, fonts, game);
        });

        // Difficulty buttons.
        build_button_section(screen, DifficultyButtonContainer, -0.5 * PI, |parent| {
            spawn_difficulty_menu_buttons(parent, fonts);
        });

        // Settings buttons and toggles.
        build_button_section(screen, SettingsButtonContainer, 0.5 * PI, |parent| {
            spawn_settings(parent, meshes, materials, fonts, settings);
        });
    });
}

fn build_button_section(
    screen: &mut ChildBuilder,
    screen_marker: impl Component,
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
                .spawn((
                    screen_marker,
                    FlexBundle::new(
                        FlexContainerStyle::default(),
                        FlexItemStyle::fixed_size(Val::Vmin(100.), Val::Vmin(100.)).with_transform(
                            Transform {
                                translation: Vec3::new(0., 2., 1.),
                                ..default()
                            },
                        ),
                    ),
                ))
                .with_children(child_builder);
        });
}

fn on_screen_change(
    mut commands: Commands,
    mut button_sections: Query<(Entity, &mut ButtonSection)>,
    main_button_container: Query<(Entity, &Children), With<MainButtonContainer>>,
    fonts: Res<Fonts>,
    game: Res<Game>,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    use ScreenState::*;
    if screen_state.0 == MainMenu {
        // Respawn buttons when going back to main screen, because the
        // Continue button may have (dis)appeared.
        for (container_entity, children) in &main_button_container {
            let mut button_container = commands.entity(container_entity);
            button_container.despawn_descendants();
            button_container.remove_children(children);
            button_container.with_children(|main_section| {
                spawn_main_menu_buttons(main_section, &fonts, &game);
            });
        }
    }

    let new_rotation = match screen_state.0 {
        MainMenu | Game => 0.,
        SelectDifficulty => 0.5 * PI,
        Settings => -0.5 * PI,
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
            Game => Animator::new(Delay::new(Duration::from_millis(200)).then(Tween::new(
                EaseMethod::Discrete(0.),
                Duration::from_millis(1),
                TransformRotationZLens { start, end },
            ))),
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

fn menu_interaction(
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

// Handles screen navigation based on button actions that persist across menus.
fn menu_button_actions(
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
