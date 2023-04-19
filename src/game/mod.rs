mod board_builder;
mod board_line_bundle;
mod board_numbers;

use self::{board_builder::build_board, board_numbers::fill_numbers};
use crate::{despawn, sudoku::Game, ScreenState, WindowSize};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::default()).add_systems((
            board_setup.in_schedule(OnEnter(ScreenState::Game)),
            despawn::<OnGameScreen>.in_schedule(OnExit(ScreenState::Game)),
        ));
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Number;

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    window_size: Res<WindowSize>,
) {
    build_board(&mut commands, &window_size);
    fill_numbers(&mut commands, &asset_server, &game, &window_size);
}
