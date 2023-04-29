mod constants;
mod game;
mod menus;
mod sudoku;
mod ui;
mod utils;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningPlugin};
use game::board_setup;
use menus::main_menu_setup;
use std::time::Duration;
use sudoku::Game;
use ui::*;
use utils::SpriteExt;

/// Screens are laid out in tiles next to one another.
#[derive(Clone, Component, Default)]
struct Screen {
    width: f32,
    height: f32,
    tile_x: f32,
    tile_y: f32,
}

impl Screen {
    fn for_state(state: ScreenState) -> Self {
        let (tile_x, tile_y) = get_tile_offset_for_screen(state);
        Self {
            tile_x,
            tile_y,
            ..default()
        }
    }
}

#[derive(Component)]
struct GameScreen;

#[derive(Component)]
struct MainScreen;

#[derive(Component)]
struct SettingsScreen;

/// State to track which screen we are in.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ScreenState {
    #[default]
    MainMenu,
    SelectDifficulty,
    Game,
    Score,
    HowToPlay,
    Options,
}

fn main() {
    App::new()
        .add_system(on_escape)
        .add_system(on_resize)
        .add_system(on_screen_change)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sudoku Pi".to_owned(),
                resolution: (480., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TweeningPlugin)
        .add_plugin(UiPlugin)
        .add_state::<ScreenState>()
        .add_startup_system(setup)
        .add_plugin(game::GamePlugin)
        .add_plugin(menus::MenuPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>) {
    commands.spawn(Camera2dBundle::default());

    let flex_container = FlexContainerBundle {
        background: Sprite::from_color(Color::WHITE),
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(100_000., 100_000., 1.),
            ..default()
        },
        ..default()
    };

    /*commands.spawn((
        Screen::with_tile_x(-1.),
        Flex,
        SettingsScreen,
        flex_container.clone(),
    ));*/

    let mut main_screen = commands.spawn((
        Screen::for_state(ScreenState::MainMenu),
        Flex,
        MainScreen,
        flex_container.clone(),
    ));
    main_menu_setup(&mut main_screen, &asset_server, &game);

    let mut game_screen = commands.spawn((
        Screen::for_state(ScreenState::Game),
        Flex,
        GameScreen,
        flex_container,
    ));
    board_setup(&mut game_screen, &asset_server, &game);
}

fn on_escape(
    input: Res<Input<KeyCode>>,
    current_state: Res<State<ScreenState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if current_state.0 == ScreenState::MainMenu {
            app_exit_events.send(AppExit);
        } else {
            next_state.set(ScreenState::MainMenu);
        }
    }
}

fn on_screen_change(
    mut commands: Commands,
    screen_state: Res<State<ScreenState>>,
    screens: Query<(Entity, &Screen, &Transform)>,
    animators: Query<Entity, With<Animator<Transform>>>,
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    for entity in &animators {
        commands.entity(entity).remove::<Animator<Transform>>();
    }

    let (offset_x, offset_y) = get_tile_offset_for_screen(screen_state.0);

    for (entity, screen, transform) in &screens {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(200),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(
                    screen.width * (screen.tile_x - offset_x),
                    screen.height * (screen.tile_y - offset_y),
                    1.,
                ),
            },
        );

        commands.entity(entity).insert(Animator::new(tween));
    }
}

fn on_resize(
    mut commands: Commands,
    mut events: EventReader<WindowResized>,
    mut screens: Query<(&mut Screen, &mut Transform)>,
    current_screen: Res<State<ScreenState>>,
    animators: Query<Entity, With<Animator<Transform>>>,
) {
    let Some(WindowResized { width, height, .. }) = events.iter().last() else {
        return;
    };

    for entity in &animators {
        commands.entity(entity).remove::<Animator<Transform>>();
    }

    for (mut screen, mut transform) in &mut screens {
        screen.width = *width;
        screen.height = *height;

        let (offset_x, offset_y) = get_tile_offset_for_screen(current_screen.0);
        transform.translation = Vec3::new(
            width * (screen.tile_x - offset_x),
            height * (screen.tile_y - offset_y),
            1.,
        );
        transform.scale = Vec3::new(*width, *height, 1.);
    }
}

fn get_tile_offset_for_screen(screen: ScreenState) -> (f32, f32) {
    use ScreenState::*;
    match screen {
        MainMenu | SelectDifficulty => (0., 0.),
        Game | Score => (1., 0.),
        HowToPlay => (0., 1.),
        Options => (-1., 0.),
    }
}
