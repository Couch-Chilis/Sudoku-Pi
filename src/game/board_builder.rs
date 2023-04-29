use super::board_numbers::fill_numbers;
use crate::{constants::*, sudoku::Game, ui::*, utils::*};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::SpriteBundle};

#[derive(Component)]
pub struct Board;

pub fn build_board(parent: &mut EntityCommands, asset_server: &AssetServer, game: &Game) {
    parent.with_children(|screen| {
        let mut board = screen.spawn((
            Board,
            FlexItemBundle::with_style(FlexItemStyle {
                flex_base: Size::all(Val::Vmin(90.)),
                flex_shrink: 1.,
                min_size: Size::all(Val::Vmin(50.)),
                preserve_aspect_ratio: true,
                ..default()
            }),
            SpriteBundle::default(),
        ));

        draw_lines(&mut board);

        fill_numbers(&mut board, asset_server, game)
    });
}

enum Orientation {
    Horizontal,
    Vertical,
}

enum Thickness {
    Thin,
    Medium,
    Thick,
}

fn draw_lines(board: &mut EntityCommands) {
    board.with_children(|parent| {
        use Orientation::*;
        use Thickness::*;

        parent.spawn(build_line(0, Horizontal, Thick));
        parent.spawn(build_line(1, Horizontal, Thin));
        parent.spawn(build_line(2, Horizontal, Thin));
        parent.spawn(build_line(3, Horizontal, Medium));
        parent.spawn(build_line(4, Horizontal, Thin));
        parent.spawn(build_line(5, Horizontal, Thin));
        parent.spawn(build_line(6, Horizontal, Medium));
        parent.spawn(build_line(7, Horizontal, Thin));
        parent.spawn(build_line(8, Horizontal, Thin));
        parent.spawn(build_line(9, Horizontal, Thick));
        parent.spawn(build_line(0, Vertical, Thick));
        parent.spawn(build_line(1, Vertical, Thin));
        parent.spawn(build_line(2, Vertical, Thin));
        parent.spawn(build_line(3, Vertical, Medium));
        parent.spawn(build_line(4, Vertical, Thin));
        parent.spawn(build_line(5, Vertical, Thin));
        parent.spawn(build_line(6, Vertical, Medium));
        parent.spawn(build_line(7, Vertical, Thin));
        parent.spawn(build_line(8, Vertical, Thin));
        parent.spawn(build_line(9, Vertical, Thick));
    });
}

fn build_line(n: u8, orientation: Orientation, thickness: Thickness) -> impl Bundle {
    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., (n as f32 - 4.5) * CELL_SIZE, 2.),
        Vertical => Vec3::new((n as f32 - 4.5) * CELL_SIZE, 0., 2.),
    };

    use Thickness::*;
    let thickness = match thickness {
        Thin => 0.05 * CELL_SIZE,
        Medium => 0.1 * CELL_SIZE,
        Thick => 0.15 * CELL_SIZE,
    };

    let scale = match orientation {
        Horizontal => Vec3::new(9.075 * CELL_SIZE, thickness, 1.),
        Vertical => Vec3::new(thickness, 9.075 * CELL_SIZE, 1.),
    };

    SpriteBundle {
        sprite: Sprite::from_color(Color::BLACK),
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        ..default()
    }
}
