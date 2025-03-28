mod difficulty_menu;
mod main_menu;
mod settings_menu;
mod settings_toggle;

use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseMethod, Lens, Tween};
use smallvec::smallvec;

use crate::{ui::*, ScreenInteraction, ScreenState};

use difficulty_menu::*;
use main_menu::*;
use settings_menu::*;
use settings_toggle::*;

pub use settings_menu::settings_screen;
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

pub fn menu_screen() -> impl FnOnce(&Props, &mut ChildBuilder) {
    fragment4(
        // Logo.
        dynamic_image(
            launch_screen,
            (
                fixed_size(Val::CrossPercent(46.19), Val::Percent(100.)),
                fixed_aspect_ratio,
                without_occupying_space,
                z_index(2.),
            ),
        ),
        // Spacer.
        leaf(fixed_size(Val::Percent(100.), Val::Percent(50.))),
        // Main menu buttons.
        button_section(ScreenState::MainMenu, 0., 2., main_menu_buttons),
        // Difficulty buttons.
        button_section(
            ScreenState::SelectDifficulty,
            -0.5 * PI,
            3.,
            difficulty_menu_buttons(),
        ),
    )
}

fn button_section(
    screen_state: ScreenState,
    initial_rotation: f32,
    z_index: f32,
    children: impl FnOnce(&Props, &mut ChildBuilder) + 'static,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder)) {
    column_t(
        ButtonSection {
            initial_rotation,
            current_rotation: initial_rotation,
        },
        (
            available_size,
            without_occupying_space,
            transform(Transform {
                rotation: Quat::from_rotation_z(initial_rotation),
                translation: Vec3::new(0., -1., z_index),
                ..default()
            }),
        ),
        padding(Sides::all(Val::Vmin(10.))),
        column_t(
            ScreenInteraction {
                screens: smallvec![screen_state],
            },
            (available_size, translation(Vec3::new(0., 2., 1.))),
            (),
            children,
        ),
    )
}

fn on_screen_change(
    mut commands: Commands,
    mut button_sections: Query<(Entity, &mut ButtonSection)>,
    button_containers: Query<(Entity, &Children, &ScreenInteraction)>,
    props: PropsTuple,
    screen_state: Res<State<ScreenState>>,
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    let props = Props::from_tuple(&props);

    use ScreenState::*;
    if screen_state.get() == &MainMenu {
        // Respawn buttons when going back to main screen, because the
        // Continue button may have (dis)appeared.
        for (container_entity, children, screen_interaction) in &button_containers {
            if screen_interaction.screens.contains(&ScreenState::MainMenu) {
                let mut button_container = commands.entity(container_entity);
                button_container.despawn_descendants();
                button_container.remove_children(children);
                button_container.with_children(|cb| main_menu_buttons(&props, cb));
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
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<FlexItemStyle>, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.transform.rotation = Quat::from_rotation_z(value);
    }
}
