#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod assets;
mod constants;
mod game;
mod highscores;
mod menus;
mod onboarding;
mod pointer_query;
mod settings;
//#[cfg(feature = "steam")]
//mod steam;
mod resource_bag;
mod sudoku;
mod transition_events;
mod ui;
mod utils;

use assets::*;
use bevy::app::AppExit;
use bevy::asset::io::memory::MemoryAssetReader;
use bevy::asset::io::{AssetSourceBuilder, AssetSourceBuilders, AssetSourceId};
use bevy::prelude::*;
use bevy::window::{WindowCloseRequested, WindowDestroyed, WindowMode, WindowResized};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningPlugin};
use game::{game_screen, highscore_screen, ActiveSliceHandles};
use highscores::Highscores;
use menus::{menu_screen, settings_screen, SettingsToggleTimer};
use onboarding::*;
use resource_bag::ResourceBag;
use settings::Settings;
use smallvec::SmallVec;
use std::time::Duration;
use sudoku::Game;
use transition_events::{on_transition, TransitionEvent};
use ui::*;

// iPhone:
const INITIAL_WIDTH: f32 = 390.;
const INITIAL_HEIGHT: f32 = 845.;

// iPad:
//const INITIAL_WIDTH: f32 = 768.;
//const INITIAL_HEIGHT: f32 = 1024.;

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
    tile_x: i8,
    tile_y: i8,
}

impl Screen {
    fn for_state(state: ScreenState) -> Self {
        let (tile_x, tile_y) = state.tile_offsets();
        Self {
            state,
            tile_x,
            tile_y,
            ..default()
        }
    }
}

/// State to track which screen we are in.
#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ScreenState {
    #[default]
    MainMenu,
    SelectDifficulty,
    Game,
    Highscores,
    Settings,
    Welcome,
    LearnNumbers,
    LearnNotes,
}

impl ScreenState {
    fn from_tile_offsets(x: i8, y: i8) -> Option<Self> {
        use ScreenState::*;
        match (x, y) {
            (0, 0) => Some(MainMenu),
            (1, 0) => Some(Game),
            (1, 1) => Some(Highscores),
            (2, 0) => Some(Settings),
            (-3, 0) => Some(Welcome),
            (-2, 0) => Some(LearnNumbers),
            (-1, 0) => Some(LearnNotes),
            _ => None,
        }
    }

    fn tile_offsets(self) -> (i8, i8) {
        use ScreenState::*;
        match self {
            MainMenu | SelectDifficulty => (0, 0),
            Game => (1, 0),
            Highscores => (1, 1),
            Settings => (2, 0),
            Welcome => (-3, 0),
            LearnNumbers => (-2, 0),
            LearnNotes => (-1, 0),
        }
    }
}

/// Overrides the screen(s) for which the given entity provides interactivity.
#[derive(Component)]
pub struct ScreenInteraction {
    screens: SmallVec<[ScreenState; 4]>,
}

#[derive(Clone, Copy, Resource)]
pub struct ScreenSizing {
    width: f32,
    height: f32,
    /// Padding to reserve space on the top screen edge, for things like status
    /// bars on mobile.
    top_padding: i32,
}

impl ScreenSizing {
    pub fn is_tablet(&self) -> bool {
        self.portrait_ratio() < 1.
    }

    /// Returns the ratio between the height and the width, where the height is
    /// assumed to be significantly larger than the width.
    ///
    /// The ratio is chosen such that phone form factors tend to have a ratio
    /// above 1.0, while most tablets will have a ratio below 1.0.
    pub fn portrait_ratio(&self) -> f32 {
        self.height / (1.6 * self.width)
    }
}

impl Default for ScreenSizing {
    fn default() -> Self {
        // Dimensions will be determined in `on_resize()`.
        Self {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            top_padding: 0,
        }
    }
}

/// Helps compensate zooming that occurs on iPhone Mini.
#[derive(Default, Resource)]
pub struct ZoomFactor {
    x: f32,
    y: f32,
}

pub fn main() {
    run(ScreenSizing::default(), ZoomFactor::default())
}

#[no_mangle]
#[cfg(target_os = "ios")]
extern "C" fn run_with_fixed_sizes(
    width: f64,
    height: f64,
    scale: f64,
    native_scale: f64,
    top_padding: i32,
) {
    let scale = (scale / native_scale) as f32;
    println!("Starting at size {width}x{height} (scale={scale}, top_padding={top_padding}px)");
    run(
        ScreenSizing {
            width: width as f32,
            height: height as f32,
            top_padding,
        },
        ZoomFactor { x: scale, y: scale },
    )
}

/// Plugin that disables all the asset loaders, since we load all assets manually.
struct AssetConfiguratorPlugin {}

impl Plugin for AssetConfiguratorPlugin {
    fn build(&self, app: &mut App) {
        let mut sources = app
            .world
            .get_resource_or_insert_with::<AssetSourceBuilders>(Default::default);
        sources.insert(
            AssetSourceId::Default,
            AssetSourceBuilder::default().with_reader(|| Box::<MemoryAssetReader>::default()),
        );
    }
}

fn run(screen_sizing: ScreenSizing, zoom_factor: ZoomFactor) {
    let settings = Settings::load();
    let game = if settings.onboarding_finished {
        Game::load()
    } else {
        Game::load_tutorial()
    };

    let mut timer = GameTimer::default();
    if game.elapsed_secs != 0. {
        timer.elapsed_secs = game.elapsed_secs;
    }

    let mut app = App::new();
    app.init_resource::<Fonts>()
        .init_resource::<Images>()
        .init_resource::<ActiveSliceHandles>()
        .insert_resource(game)
        .insert_resource(timer)
        .insert_resource(settings)
        .insert_resource(Highscores::load())
        .insert_resource(SettingsToggleTimer::default())
        .insert_resource(screen_sizing)
        .insert_resource(zoom_factor)
        .insert_resource(ClearColor(Color::WHITE))
        .init_state::<ScreenState>()
        .add_event::<TransitionEvent>()
        .add_event::<WindowDestroyed>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                on_escape,
                #[cfg(not(platform = "ios"))]
                on_resize,
                #[cfg(debug_assertions)]
                on_keyboard_input,
                on_screen_change,
                on_window_close,
                on_exit.after(on_window_close),
                onboarding_screen_button_interaction,
                how_to_play_numbers_interaction,
                how_to_play_notes_interaction,
                on_transition,
            ),
        )
        .add_plugins(AssetConfiguratorPlugin {})
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sudoku Pi".to_owned(),
                resolution: (INITIAL_WIDTH, INITIAL_HEIGHT).into(),
                mode: get_initial_window_mode(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            FramepacePlugin,
            TweeningPlugin,
            UiPlugin,
            game::GamePlugin,
            menus::MenuPlugin,
        ));

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    add_steamworks_plugin(&mut app);

    app.run();
}

// #[cfg(feature = "steam")]
// fn add_steamworks_plugin(app: &mut App) {
//     use bevy_steamworks::*;
//     app.add_plugins(SteamworksPlugin::new(AppId(892884)));
// }

// #[cfg(not(feature = "steam"))]
fn add_steamworks_plugin(_app: &mut App) {}

#[cfg(debug_assertions)]
fn on_keyboard_input(
    mut screen_state: ResMut<NextState<ScreenState>>,
    current_screen: Res<State<ScreenState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.all_pressed([KeyCode::ControlLeft, KeyCode::AltLeft]) {
        return;
    }

    let mut move_screen = |current_screen: &ScreenState, dx, dy| {
        let (x, y) = current_screen.tile_offsets();
        if let Some(new_screen) = ScreenState::from_tile_offsets(x + dx, y + dy) {
            screen_state.set(new_screen);
        }
    };

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => move_screen(current_screen.get(), 0, 1),
            ArrowRight => move_screen(current_screen.get(), 1, 0),
            ArrowDown => move_screen(current_screen.get(), 0, -1),
            ArrowLeft => move_screen(current_screen.get(), -1, 0),
            _ => {}
        }
    }
}

fn setup(
    mut commands: Commands,
    fonts: ResMut<Assets<Font>>,
    mut framepace_settings: ResMut<FramepaceSettings>,
    images: ResMut<Assets<Image>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    settings: Res<Settings>,
    game: Res<Game>,
    highscores: Res<Highscores>,
    screen_sizing: Res<ScreenSizing>,
) {
    commands.spawn(Camera2dBundle::default());

    framepace_settings.limiter = Limiter::from_framerate(60.0);

    let fonts = Fonts::load(fonts);
    let fortune = Fortune::load();
    let images = Images::load(images);

    let props = Props {
        game: &game,
        highscores: &highscores,
        resources: ResourceBag {
            fonts: &fonts,
            images: &images,
            screen_sizing: &screen_sizing,
        },
        settings: &settings,
    };

    let resources = &props.resources;

    use ScreenState::*;
    commands.spawn_with_children(&props, screen(MainMenu, resources, menu_screen()));
    commands.spawn_with_children(&props, screen(Game, resources, game_screen()));
    commands.spawn_with_children(&props, screen(Highscores, resources, highscore_screen()));
    commands.spawn_with_children(&props, screen(Settings, resources, settings_screen()));
    commands.spawn_with_children(&props, screen(Welcome, resources, welcome_screen()));
    commands.spawn_with_children(&props, screen(LearnNotes, resources, learn_notes_screen()));
    commands.spawn_with_children(
        &props,
        screen(LearnNumbers, resources, learn_numbers_screen()),
    );

    if !settings.onboarding_finished {
        screen_state.set(Welcome);
    }

    commands.insert_resource(ActiveSliceHandles::load(&images));
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
    input: Res<ButtonInput<KeyCode>>,
    mut transition_events: EventWriter<TransitionEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        transition_events.send(TransitionEvent::Exit);
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

    let (offset_x, offset_y) = screen_state.get().tile_offsets();

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
                    screen.width * (screen.tile_x - offset_x) as f32,
                    screen.height * (screen.tile_y - offset_y) as f32,
                    1.,
                ),
            },
        );

        commands.entity(entity).insert(Animator::new(tween));
    }
}

#[cfg(not(platform = "ios"))]
fn on_resize(
    mut commands: Commands,
    mut events: EventReader<WindowResized>,
    mut screens: Query<(&mut Screen, &mut Transform)>,
    mut screen_sizing: ResMut<ScreenSizing>,
    current_screen: Res<State<ScreenState>>,
    animators: Query<Entity, With<Animator<Transform>>>,
) {
    let Some(WindowResized { width, height, .. }) = events.read().last() else {
        return;
    };

    for entity in &animators {
        commands.entity(entity).remove::<Animator<Transform>>();
    }

    screen_sizing.width = *width;
    screen_sizing.height = *height;

    for (mut screen, mut transform) in &mut screens {
        screen.width = *width;
        screen.height = *height;

        let (offset_x, offset_y) = current_screen.get().tile_offsets();
        transform.translation = Vec3::new(
            width * (screen.tile_x - offset_x) as f32,
            height * (screen.tile_y - offset_y) as f32,
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

fn screen<B>(
    screen: ScreenState,
    resources: &ResourceBag,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let (mut bundle, spawn_children) = container(
        (screen_gap(screen), screen_padding(resources, screen)),
        child,
    );

    let screen_sizing = resources.screen_sizing;
    let (tile_x, tile_y) = screen.tile_offsets();
    bundle.transform = Transform {
        scale: Vec3::new(screen_sizing.width, screen_sizing.height, 1.),
        translation: Vec3::new(
            screen_sizing.width * tile_x as f32,
            screen_sizing.height * tile_y as f32,
            1.,
        ),
        ..default()
    };

    ((Screen::for_state(screen), Flex, bundle), spawn_children)
}
