mod constants;
mod game;
mod menus;
mod sudoku;
mod utils;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowResized;

/// State to track which screen we are in.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ScreenState {
    #[default]
    Splash,
    MainMenu,
    SelectDifficulty,
    Game,
    Score,
    HowToPlay,
    Options,
}

#[derive(Clone, Copy, Debug, Default, Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
    pub vmin_scale: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowSize::default())
        .add_system(quit_on_escape)
        .add_system(on_resize)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sudoku Pi".to_owned(),
                resolution: (480., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<ScreenState>()
        .add_system(skip_splash_screen.in_schedule(OnEnter(ScreenState::Splash)))
        .add_startup_system(setup)
        .add_plugin(menus::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.jpg"),
        transform: Transform {
            scale: Vec3::new(1., 1., 0.),
            ..default()
        },
        ..default()
    });
}

fn quit_on_escape(input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn return_to_menu_on_escape(
    input: Res<Input<KeyCode>>,
    mut screen_state: ResMut<NextState<ScreenState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        screen_state.set(ScreenState::MainMenu);
    }
}

fn skip_splash_screen(mut screen_state: ResMut<NextState<ScreenState>>) {
    screen_state.set(ScreenState::MainMenu);
}

fn on_resize(
    mut events: EventReader<WindowResized>,
    mut screen_state: ResMut<NextState<ScreenState>>,
    mut window_size: ResMut<WindowSize>,
) {
    for &WindowResized { width, height, .. } in events.iter() {
        window_size.width = width;
        window_size.height = height;
        window_size.vmin_scale = 0.01 * if width < height { width } else { height };
        screen_state.set(ScreenState::Splash);
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
