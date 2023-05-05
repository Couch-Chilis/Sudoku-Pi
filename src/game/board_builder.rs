use super::{board_numbers::fill_numbers, wheel::init_wheel};
use crate::{constants::*, sudoku::Game, ui::*, utils::*, Fonts, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::SpriteBundle};

#[derive(Component)]
pub struct Board;

pub fn build_board(
    parent: &mut EntityCommands,
    asset_server: &AssetServer,
    fonts: &Fonts,
    game: &Game,
    settings: &Settings,
) {
    parent.with_children(|screen| {
        let mut board = screen.spawn((
            Board,
            FlexLeafBundle::from_style(
                FlexItemStyle::preferred_and_minimum_size(
                    Size::all(Val::Vmin(90.)),
                    Size::all(Val::Vmin(50.)),
                )
                .with_fixed_aspect_ratio(),
            ),
        ));

        draw_lines(&mut board);

        fill_numbers(&mut board, fonts, game, settings);

        init_wheel(&mut board, asset_server);
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
    use Thickness::*;
    let (thickness, color, z) = match thickness {
        Thin => (0.05 * CELL_SIZE, COLOR_BOARD_LINE_THIN, 2.),
        Medium => (0.1 * CELL_SIZE, COLOR_BOARD_LINE_MEDIUM, 3.),
        Thick => (0.15 * CELL_SIZE, COLOR_BOARD_LINE_THICK, 4.),
    };

    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., (n as f32 - 4.5) * CELL_SIZE, z),
        Vertical => Vec3::new((n as f32 - 4.5) * CELL_SIZE, 0., z),
    };

    let scale = match orientation {
        Horizontal => Vec3::new(9.075 * CELL_SIZE, thickness, 1.),
        Vertical => Vec3::new(thickness, 9.075 * CELL_SIZE, 1.),
    };

    SpriteBundle {
        sprite: Sprite::from_color(color),
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        ..default()
    }
}
