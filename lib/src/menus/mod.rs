mod difficulty_menu;
mod how_to_play;
mod main_menu;
mod settings_menu;
mod settings_toggle;

use crate::{
    sudoku::*, ui::*, utils::*, Fonts, Images, ScreenInteraction, ScreenSizing, ScreenState,
};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseFunction, EaseMethod, Lens, Tween};
use difficulty_menu::*;
use main_menu::*;
use settings_menu::*;
use settings_toggle::*;
use smallvec::smallvec;
use std::f32::consts::PI;
use std::time::Duration;

pub use settings_menu::settings_screen_setup;
pub use settings_toggle::SettingsToggleTimer;

#[derive(Component, Default)]
struct ButtonSection {
    initial_rotation: f32,
    current_rotation: f32,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                difficulty_screen_button_actions.run_if(in_state(ScreenState::SelectDifficulty)),
                main_menu_button_actions.run_if(in_state(ScreenState::MainMenu)),
                settings_screen_button_actions.run_if(in_state(ScreenState::Settings)),
                settings_toggle_actions.run_if(in_state(ScreenState::Settings)),
                on_setting_change,
                render_settings_toggles.run_if(in_state(ScreenState::Settings)),
                on_screen_change.before(LayoutSystem::ApplyLayout),
            ),
        );
    }
}

pub fn menu_setup(
    main_screen: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    screen_sizing: &ScreenSizing,
) {
    main_screen.with_children(|screen| {
        // Logo.
        screen.spawn((
            FlexItemBundle::from_style(
                FlexItemStyle::fixed_size(Val::CrossPercent(46.19), Val::Percent(100.))
                    .with_fixed_aspect_ratio()
                    .without_occupying_space()
                    .with_transform(Transform::from_2d_scale(1. / 1170., 1. / 2533.)),
            ),
            SpriteBundle {
                texture: if screen_sizing.is_ipad {
                    images.launch_screen_ipad.clone()
                } else {
                    images.launch_screen.clone()
                },
                ..default()
            },
        ));

        // Spacer.
        screen.spawn(FlexLeafBundle::from_style(
            FlexItemStyle::fixed_size(Val::Percent(100.), Val::Percent(50.))
                .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.))),
        ));

        let buttons = make_button_builder(fonts, screen_sizing);

        // Main menu buttons.
        build_button_section(screen, ScreenState::MainMenu, 0., 2., |main_section| {
            spawn_main_menu_buttons(main_section, &buttons, fonts, game);
        });

        // Difficulty buttons.
        build_button_section(
            screen,
            ScreenState::SelectDifficulty,
            -0.5 * PI,
            3.,
            |parent| {
                spawn_difficulty_menu_buttons(parent, &buttons);
            },
        );
    });
}

fn make_button_builder(fonts: &Fonts, screen_sizing: &ScreenSizing) -> ButtonBuilder {
    let button_style = if screen_sizing.is_ipad {
        FlexItemStyle::fixed_size(Val::Pixel(600), Val::Pixel(60))
            .with_margin(Size::all(Val::Vmin(1.5)))
    } else {
        FlexItemStyle::fixed_size(Val::Vmin(70.), Val::Vmin(10.))
            .with_margin(Size::all(Val::Vmin(1.5)))
    };
    let font_size = if screen_sizing.is_ipad { 66. } else { 44. };
    ButtonBuilder::new(fonts, button_style, font_size)
}

fn build_button_section(
    screen: &mut ChildBuilder,
    screen_state: ScreenState,
    initial_rotation: f32,
    z_index: f32,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    screen
        .spawn((
            ButtonSection {
                initial_rotation,
                current_rotation: initial_rotation,
            },
            FlexBundle::new(
                FlexItemStyle::available_size()
                    .without_occupying_space()
                    .with_transform(Transform {
                        rotation: Quat::from_rotation_z(initial_rotation),
                        translation: Vec3::new(0., -1., z_index),
                        ..default()
                    }),
                FlexContainerStyle::default().with_padding(Sides::all(Val::Vmin(10.))),
            ),
        ))
        .with_children(|main_section_rotation_axis| {
            main_section_rotation_axis
                .spawn((
                    ScreenInteraction {
                        screens: smallvec![screen_state],
                    },
                    FlexBundle::from_item_style(FlexItemStyle::available_size().with_transform(
                        Transform {
                            translation: Vec3::new(0., 2., 1.),
                            ..default()
                        },
                    )),
                ))
                .with_children(child_builder);
        });
}

fn on_screen_change(
    mut commands: Commands,
    mut button_sections: Query<(Entity, &mut ButtonSection)>,
    button_containers: Query<(Entity, &Children, &ScreenInteraction)>,
    fonts: Res<Fonts>,
    game: Res<Game>,
    screen_sizing: Res<ScreenSizing>,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    use ScreenState::*;
    if screen_state.get() == &MainMenu {
        // Respawn buttons when going back to main screen, because the
        // Continue button may have (dis)appeared.
        for (container_entity, children, screen_interaction) in &button_containers {
            if screen_interaction.screens.contains(&ScreenState::MainMenu) {
                let mut button_container = commands.entity(container_entity);
                button_container.despawn_descendants();
                button_container.remove_children(children);
                button_container.with_children(|main_section| {
                    let buttons = make_button_builder(&fonts, &screen_sizing);
                    spawn_main_menu_buttons(main_section, &buttons, &fonts, &game);
                });
            }
        }
    }

    let new_rotation = match screen_state.get() {
        MainMenu | Game => 0.,
        SelectDifficulty => 0.5 * PI,
        _ => return,
    };

    for (entity, mut button_section) in &mut button_sections {
        let start = button_section.current_rotation;
        let end = button_section.initial_rotation + new_rotation;
        if start == end {
            continue;
        }

        let animator = match screen_state.get() {
            // When going from the difficulty selection to the game, we just
            // reset the transform without animation so everything is back to
            // the starting position when going out of the game.
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

impl Lens<FlexItemStyle> for TransformRotationZLens {
    fn lerp(&mut self, target: &mut FlexItemStyle, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.transform.rotation = Quat::from_rotation_z(value);
    }
}
