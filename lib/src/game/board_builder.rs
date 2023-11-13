use super::{board_numbers::fill_numbers, wheel::init_wheel};
use crate::{constants::*, sudoku::Game, ui::*, utils::*, ResourceBag, ScreenState, Settings};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::SpriteBundle};

#[derive(Component)]
pub struct Board;

#[derive(Clone, Component, Default)]
pub struct MistakeCellBorders;

#[derive(Bundle, Clone, Default)]
pub struct MistakeCellBordersBundle {
    pub marker: MistakeCellBorders,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

pub fn build_board(
    parent: &mut ChildBuilder,
    game: &Game,
    resources: &ResourceBag,
    screen: ScreenState,
    settings: &Settings,
) {
    let mut board = parent.spawn((
        Board,
        screen,
        FlexBundle::new(
            if resources.screen_sizing.is_ipad {
                FlexItemStyle::preferred_and_minimum_size(
                    Size::all(Val::Vmin(80.)),
                    Size::all(Val::Vmin(50.)),
                )
            } else {
                FlexItemStyle::preferred_and_minimum_size(
                    Size::all(Val::Vmin(90.)),
                    Size::all(Val::Vmin(50.)),
                )
            }
            .with_fixed_aspect_ratio(),
            FlexContainerStyle::row(),
        ),
    ));

    draw_lines(&mut board);

    fill_numbers(&mut board, game, resources, settings);

    init_wheel(&mut board, resources, screen);

    init_mistake_borders(&mut board);
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

fn init_mistake_borders(board: &mut EntityCommands) {
    board.with_children(|parent| {
        parent
            .spawn(MistakeCellBordersBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 8.)),
                visibility: Visibility::Hidden,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(build_mistake_line(-0.5, Orientation::Horizontal));
                parent.spawn(build_mistake_line(0.5, Orientation::Horizontal));
                parent.spawn(build_mistake_line(-0.5, Orientation::Vertical));
                parent.spawn(build_mistake_line(0.5, Orientation::Vertical));
            });
    });
}

fn build_mistake_line(edge: f32, orientation: Orientation) -> impl Bundle {
    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., edge * CELL_SIZE, 1.),
        Vertical => Vec3::new(edge * CELL_SIZE, 0., 1.),
    };

    let scale = match orientation {
        Horizontal => Vec3::new(1.03 * CELL_SIZE, 0.06 * CELL_SIZE, 1.),
        Vertical => Vec3::new(0.06 * CELL_SIZE, 1.03 * CELL_SIZE, 1.),
    };

    SpriteBundle {
        sprite: Sprite::from_color(COLOR_POP_DARK),
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        ..default()
    }
}
