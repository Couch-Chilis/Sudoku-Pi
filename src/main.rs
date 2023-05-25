#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod constants;
mod game;
mod highscores;
mod menus;
mod settings;
mod sudoku;
mod ui;
mod utils;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::window::{WindowMode, WindowResized};
use bevy::{app::AppExit, time::Stopwatch};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningPlugin};
use game::{board_setup, highscore_screen_setup};
use highscores::Highscores;
use menus::menu_setup;
use settings::Settings;
use std::time::Duration;
use sudoku::Game;
use ui::*;
use utils::{SpriteExt, TransformExt};

const BOLD_FONT: &[u8] = include_bytes!("../assets/Tajawal/Tajawal-Bold.ttf");
const MEDIUM_FONT: &[u8] = include_bytes!("../assets/Tajawal/Tajawal-Medium.ttf");
//const REGULAR_FONT: &[u8] = include_bytes!("../assets/Tajawal/Tajawal-Regular.ttf");

const COG: &[u8] = include_bytes!("../assets/cog.png");
const COG_PRESSED: &[u8] = include_bytes!("../assets/cog_pressed.png");
const LOGO: &[u8] = include_bytes!("../assets/logo.png");

#[derive(Clone, Default, Resource)]
pub struct Fonts {
    bold: Handle<Font>,
    medium: Handle<Font>,
    //regular: Handle<Font>,
}

#[derive(Clone, Default, Resource)]
pub struct Images {
    cog: Handle<Image>,
    cog_pressed: Handle<Image>,
    logo: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct GameTimer {
    stopwatch: Stopwatch,
}

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

/// State to track which screen we are in.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ScreenState {
    #[default]
    MainMenu,
    SelectDifficulty,
    Game,
    Highscores,
    HowToPlay,
    Settings,
    Upper,
}

fn main() {
    let game = Game::load();

    let mut timer = GameTimer::default();
    if game.elapsed_secs != 0. {
        timer
            .stopwatch
            .set_elapsed(Duration::from_secs_f32(game.elapsed_secs));
    }

    App::new()
        .init_resource::<Fonts>()
        .init_resource::<Images>()
        .insert_resource(game)
        .insert_resource(timer)
        .insert_resource(Settings::load())
        .insert_resource(Highscores::load())
        .add_startup_system(setup)
        .add_system(on_escape)
        .add_system(on_resize)
        .add_system(on_screen_change)
        .add_system(on_before_exit)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sudoku Pi".to_owned(),
                resolution: (480., 720.).into(),
                mode: get_initial_window_mode(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TweeningPlugin)
        .add_plugin(UiPlugin)
        .add_state::<ScreenState>()
        .add_plugin(game::GamePlugin)
        .add_plugin(menus::MenuPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    game: Res<Game>,
    highscores: Res<Highscores>,
) {
    commands.spawn(Camera2dBundle::default());

    let fonts = Fonts {
        bold: fonts.add(Font::try_from_bytes(Vec::from(BOLD_FONT)).unwrap()),
        medium: fonts.add(Font::try_from_bytes(Vec::from(MEDIUM_FONT)).unwrap()),
        //regular: fonts.add(Font::try_from_bytes(Vec::from(REGULAR_FONT)).unwrap()),
    };

    let images = Images {
        cog: images.add(load_png(COG)),
        cog_pressed: images.add(load_png(COG_PRESSED)),
        logo: images.add(load_png(LOGO)),
    };

    let mut main_screen = spawn_screen(&mut commands, ScreenState::MainMenu);
    menu_setup(
        &mut main_screen,
        &mut meshes,
        &mut materials,
        &images,
        &settings,
        &fonts,
        &game,
    );

    let mut game_screen = spawn_screen(&mut commands, ScreenState::Game);
    board_setup(
        &mut game_screen,
        &mut meshes,
        &mut materials,
        &asset_server,
        &fonts,
        &game,
        &settings,
    );

    let mut highscore_screen = spawn_screen(&mut commands, ScreenState::Highscores);
    highscore_screen_setup(
        &mut highscore_screen,
        &mut meshes,
        &mut materials,
        &fonts,
        &game,
        &highscores,
    );

    // This screen is just there so there is no empty space in the transition
    // from highscore back to the main menu.
    commands.spawn((
        Screen::for_state(ScreenState::Upper),
        SpriteBundle {
            sprite: Sprite::from_color(Color::WHITE),
            ..default()
        },
    ));

    commands.insert_resource(fonts);
    commands.insert_resource(images);
}

// Synchronize the timer to the game state right before the game exits.
// We don't keep the timer in the game state updated all the time, because it
// would trigger full rerenders of the board every frame.
fn on_before_exit(
    mut game: ResMut<Game>,
    game_timer: Res<GameTimer>,
    exit_events: EventReader<AppExit>,
) {
    if !exit_events.is_empty() {
        game.elapsed_secs = game_timer.stopwatch.elapsed_secs();
    }
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
) {
    if !screen_state.is_changed() || screen_state.is_added() {
        return;
    }

    let (offset_x, offset_y) = get_tile_offset_for_screen(screen_state.0);

    for (entity, screen, transform) in &screens {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(if screen_state.0 == ScreenState::Highscores {
                2000
            } else {
                200
            }),
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

fn get_initial_window_mode() -> WindowMode {
    if std::env::var_os("SteamTenfoot").is_some() {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    }
}

fn get_tile_offset_for_screen(screen: ScreenState) -> (f32, f32) {
    use ScreenState::*;
    match screen {
        MainMenu | SelectDifficulty | Settings => (0., 0.),
        Game => (1., 0.),
        Highscores => (1., 1.),
        HowToPlay => (-1., 0.),
        Upper => (0., 1.),
    }
}

fn load_png(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
    )
    .unwrap()
}

fn spawn_screen<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    screen: ScreenState,
) -> EntityCommands<'w, 's, 'a> {
    let flex_container = FlexContainerBundle {
        background: Sprite::from_color(Color::WHITE),
        transform: Transform::from_2d_scale(100_000., 100_000.),
        ..default()
    };

    commands.spawn((Screen::for_state(screen), Flex, flex_container))
}
