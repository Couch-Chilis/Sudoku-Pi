#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod constants;
mod game;
mod highscores;
mod menus;
mod pointer_query;
mod settings;
#[cfg(feature = "steam")]
mod steam;
mod sudoku;
mod ui;
mod utils;

use bevy::app::AppExit;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::window::{WindowCloseRequested, WindowDestroyed, WindowMode, WindowResized};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningPlugin};
use game::{board_setup, highscore_screen_setup, SliceHandles};
use highscores::Highscores;
use menus::{menu_setup, settings_screen_setup};
use settings::Settings;
use smallvec::SmallVec;
use std::time::Duration;
use sudoku::Game;
use ui::*;
use utils::{SpriteExt, TransformExt};

const BOLD_FONT: &[u8] = include_bytes!("../../assets/Tajawal/Tajawal-Bold.ttf");
const MEDIUM_FONT: &[u8] = include_bytes!("../../assets/Tajawal/Tajawal-Medium.ttf");
const LIGHT_FONT: &[u8] = include_bytes!("../../assets/Tajawal/Tajawal-Light.ttf");

const XIAOWEI_REGULAR_FONT: &[u8] = include_bytes!("../../assets/XiaoWei/ZCOOLXiaoWei-Regular.ttf");

const COG: &[u8] = include_bytes!("../../assets/cog.png");
const COG_PRESSED: &[u8] = include_bytes!("../../assets/cog_pressed.png");
const LOGO: &[u8] = include_bytes!("../../assets/logo.png");
const MODE_SLIDER: &[u8] = include_bytes!("../../assets/mode_slider.png");
const SCROLL: &[u8] = include_bytes!("../../assets/scroll.png");
const SLICE_1: &[u8] = include_bytes!("../../assets/slice_1.png");
const SLICE_2: &[u8] = include_bytes!("../../assets/slice_2.png");
const SLICE_3: &[u8] = include_bytes!("../../assets/slice_3.png");
const SLICE_4: &[u8] = include_bytes!("../../assets/slice_4.png");
const SLICE_5: &[u8] = include_bytes!("../../assets/slice_5.png");
const SLICE_6: &[u8] = include_bytes!("../../assets/slice_6.png");
const SLICE_7: &[u8] = include_bytes!("../../assets/slice_7.png");
const SLICE_8: &[u8] = include_bytes!("../../assets/slice_8.png");
const SLICE_9: &[u8] = include_bytes!("../../assets/slice_9.png");
const TOP_LABEL: &[u8] = include_bytes!("../../assets/top_label.png");
const WALL: &[u8] = include_bytes!("../../assets/wall.png");
const WHEEL: &[u8] = include_bytes!("../../assets/wheel.png");

const FORTUNE: &[u8] = include_bytes!("../../assets/fortune.txt");

#[derive(Clone, Default, Resource)]
pub struct Fonts {
    bold: Handle<Font>,
    medium: Handle<Font>,
    light: Handle<Font>,

    scroll: Handle<Font>,
}

#[derive(Clone, Default, Resource)]
pub struct Fortune {
    lines: Vec<&'static str>,
}

#[derive(Clone, Default, Resource)]
pub struct Images {
    cog: Handle<Image>,
    cog_pressed: Handle<Image>,
    logo: Handle<Image>,
    mode_slider: Handle<Image>,
    scroll: Handle<Image>,
    slice_1: Handle<Image>,
    slice_2: Handle<Image>,
    slice_3: Handle<Image>,
    slice_4: Handle<Image>,
    slice_5: Handle<Image>,
    slice_6: Handle<Image>,
    slice_7: Handle<Image>,
    slice_8: Handle<Image>,
    slice_9: Handle<Image>,
    top_label: Handle<Image>,
    wall: Handle<Image>,
    wheel: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct GameTimer {
    elapsed_secs: f32,
}

/// Screens are laid out in tiles next to one another.
#[derive(Clone, Component, Default)]
struct Screen {
    state: ScreenState,
    width: f32,
    height: f32,
    tile_x: f32,
    tile_y: f32,
}

impl Screen {
    fn for_state(state: ScreenState) -> Self {
        let (tile_x, tile_y) = get_tile_offset_for_screen(state);
        Self {
            state,
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

/// Overrides the screen(s) for which the given entity provides interactivity.
#[derive(Component)]
pub struct ScreenInteraction {
    screens: SmallVec<[ScreenState; 4]>,
}

/// Padding to reserve space on the screen edges, for things like status bars
/// on mobile.
#[derive(Clone, Default, Resource)]
pub struct ScreenPadding {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

/// Helps compensate zooming that occurs on iPhone Mini.
#[derive(Default, Resource)]
pub struct ZoomFactor {
    x: f32,
    y: f32,
}

pub fn main() {
    run(ScreenPadding::default(), ZoomFactor::default())
}

#[no_mangle]
#[cfg(target_os = "ios")]
extern "C" fn run_with_scales_and_padding(scale: f64, native_scale: f64, top_padding: f64) {
    let scale = (scale / native_scale) as f32;
    run(
        ScreenPadding {
            top: top_padding as f32,
            right: 0.,
            bottom: 0.,
            left: 0.,
        },
        ZoomFactor { x: scale, y: scale },
    )
}

fn run(screen_padding: ScreenPadding, zoom_factor: ZoomFactor) {
    let game = Game::load();

    let mut timer = GameTimer::default();
    if game.elapsed_secs != 0. {
        timer.elapsed_secs = game.elapsed_secs;
    }

    let mut app = App::new();
    app.init_resource::<Fonts>()
        .init_resource::<Images>()
        .init_resource::<SliceHandles>()
        .insert_resource(game)
        .insert_resource(timer)
        .insert_resource(Settings::load())
        .insert_resource(Highscores::load())
        .insert_resource(screen_padding)
        .insert_resource(zoom_factor)
        .add_event::<WindowDestroyed>()
        .add_state::<ScreenState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                on_escape,
                on_resize,
                on_screen_change,
                on_window_close,
                on_exit.after(on_window_close),
            ),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            close_when_requested: false,
            primary_window: Some(Window {
                title: "Sudoku Pi".to_owned(),
                resolution: (390., 845.).into(),
                mode: get_initial_window_mode(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            TweeningPlugin,
            UiPlugin,
            game::GamePlugin,
            menus::MenuPlugin,
        ));

    add_steamworks_plugin(&mut app);

    app.run();
}

#[cfg(feature = "steam")]
fn add_steamworks_plugin(app: &mut App) {
    use bevy_steamworks::*;
    app.add_plugins(SteamworksPlugin::new(AppId(892884)));
}

#[cfg(not(feature = "steam"))]
fn add_steamworks_plugin(_app: &mut App) {}

fn setup(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
    game: Res<Game>,
    highscores: Res<Highscores>,
) {
    commands.spawn(Camera2dBundle::default());

    let fonts = Fonts {
        bold: fonts.add(Font::try_from_bytes(Vec::from(BOLD_FONT)).unwrap()),
        medium: fonts.add(Font::try_from_bytes(Vec::from(MEDIUM_FONT)).unwrap()),
        light: fonts.add(Font::try_from_bytes(Vec::from(LIGHT_FONT)).unwrap()),
        scroll: fonts.add(Font::try_from_bytes(Vec::from(XIAOWEI_REGULAR_FONT)).unwrap()),
    };

    let fortune = Fortune {
        lines: FORTUNE
            .split(|&c| c == b'\n')
            .map(|slice| std::str::from_utf8(slice).unwrap())
            .filter(|string| !string.is_empty())
            .collect(),
    };

    let images = Images {
        cog: images.add(load_png(COG)),
        cog_pressed: images.add(load_png(COG_PRESSED)),
        logo: images.add(load_png(LOGO)),
        mode_slider: images.add(load_png(MODE_SLIDER)),
        scroll: images.add(load_png(SCROLL)),
        slice_1: images.add(load_png(SLICE_1)),
        slice_2: images.add(load_png(SLICE_2)),
        slice_3: images.add(load_png(SLICE_3)),
        slice_4: images.add(load_png(SLICE_4)),
        slice_5: images.add(load_png(SLICE_5)),
        slice_6: images.add(load_png(SLICE_6)),
        slice_7: images.add(load_png(SLICE_7)),
        slice_8: images.add(load_png(SLICE_8)),
        slice_9: images.add(load_png(SLICE_9)),
        top_label: images.add(load_png(TOP_LABEL)),
        wall: images.add(load_png(WALL)),
        wheel: images.add(load_png(WHEEL)),
    };

    let mut main_screen = spawn_screen(&mut commands, ScreenState::MainMenu);
    menu_setup(&mut main_screen, &fonts, &game, &images);

    let mut game_screen = spawn_screen(&mut commands, ScreenState::Game);
    board_setup(
        &mut game_screen,
        &mut meshes,
        &mut materials,
        &fonts,
        &game,
        &images,
        &settings,
    );

    let mut highscore_screen = spawn_screen(&mut commands, ScreenState::Highscores);
    highscore_screen_setup(&mut highscore_screen, &fonts, &game, &highscores, &images);

    let mut settings_screen = spawn_screen(&mut commands, ScreenState::Settings);
    settings_screen_setup(
        &mut settings_screen,
        &mut meshes,
        &mut materials,
        &fonts,
        &settings,
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

    commands.insert_resource(SliceHandles::load(&images));
    commands.insert_resource(fonts);
    commands.insert_resource(fortune);
    commands.insert_resource(images);
}

// Synchronize the timer to the game state right before the game exits.
// We don't keep the timer in the game state updated all the time, because it
// would trigger full rerenders of the board every frame.
fn on_exit(
    mut game: ResMut<Game>,
    game_timer: Res<GameTimer>,
    app_exit_events: EventReader<AppExit>,
    destroyed_events: EventReader<WindowDestroyed>,
) {
    if !app_exit_events.is_empty() || !destroyed_events.is_empty() {
        println!("Saving before exit");
        game.elapsed_secs = game_timer.elapsed_secs;
        game.save();
    }
}

fn on_escape(
    input: Res<Input<KeyCode>>,
    current_state: Res<State<ScreenState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if current_state.get() == &ScreenState::MainMenu {
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

    let (offset_x, offset_y) = get_tile_offset_for_screen(*screen_state.get());

    for (entity, screen, transform) in &screens {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(if screen_state.get() == &ScreenState::Highscores {
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

        let (offset_x, offset_y) = get_tile_offset_for_screen(*current_screen.get());
        transform.translation = Vec3::new(
            width * (screen.tile_x - offset_x),
            height * (screen.tile_y - offset_y),
            1.,
        );
        transform.scale = Vec3::new(*width, *height, 1.);
    }
}

fn on_window_close(
    mut app_exit_events: EventWriter<AppExit>,
    window_close_events: EventReader<WindowCloseRequested>,
) {
    if !window_close_events.is_empty() {
        app_exit_events.send(AppExit);
    }
}

fn get_initial_window_mode() -> WindowMode {
    if cfg!(target_os = "ios") || std::env::var_os("SteamTenfoot").is_some() {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    }
}

fn get_tile_offset_for_screen(screen: ScreenState) -> (f32, f32) {
    use ScreenState::*;
    match screen {
        MainMenu | SelectDifficulty => (0., 0.),
        Game => (1., 0.),
        Highscores => (1., 1.),
        HowToPlay => (-1., 0.),
        Settings => (2., 0.),
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
        style: if screen == ScreenState::Game {
            FlexContainerStyle::default().with_gap(Val::Auto)
        } else {
            FlexContainerStyle::default()
        },
        ..default()
    };

    commands.spawn((Screen::for_state(screen), Flex, flex_container))
}
