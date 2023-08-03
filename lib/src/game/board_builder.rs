use super::{board_numbers::fill_numbers, wheel::init_wheel};
use crate::{constants::*, sudoku::Game, ui::*, utils::*, Fonts, Images, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::SpriteBundle};

#[derive(Component)]
pub struct Board;

pub fn build_board(
    parent: &mut EntityCommands,
    fonts: &Fonts,
    game: &Game,
    images: &Images,
    settings: &Settings,
) {
    parent.with_children(|screen| {
        let mut board = screen.spawn((
            Board,
            FlexBundle::new(
                FlexItemStyle::preferred_and_minimum_size(
                    Size::all(Val::Vmin(90.)),
                    Size::all(Val::Vmin(50.)),
                )
                .with_fixed_aspect_ratio(),
                FlexContainerStyle::row(),
            ),
        ));

        draw_lines(&mut board);

        fill_numbers(&mut board, fonts, game, settings);

        init_wheel(&mut board, images, fonts);
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
        Thin => (0.03 * CELL_SIZE, COLOR_BOARD_LINE_THIN, 5.),
        Medium => (0.03 * CELL_SIZE, COLOR_BOARD_LINE_MEDIUM, 6.),
        Thick => (0.06 * CELL_SIZE, COLOR_BOARD_LINE_THICK, 7.),
    };

    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., (n as f32 - 4.5) * CELL_SIZE, z),
        Vertical => Vec3::new((n as f32 - 4.5) * CELL_SIZE, 0., z),
    };

    let scale = match orientation {
        Horizontal => Vec3::new(9.03 * CELL_SIZE, thickness, 1.),
        Vertical => Vec3::new(thickness, 9.03 * CELL_SIZE, 1.),
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
