use bevy::prelude::*;

use crate::{constants::*, ui::*, ScreenState};

use super::{board_numbers, wheel::wheel};

#[derive(Component)]
pub struct Board;

#[derive(Clone, Component, Default)]
pub struct MistakeCellBorders;

#[derive(Bundle, Clone, Default)]
pub struct MistakeCellBordersBundle {
    pub marker: MistakeCellBorders,
    pub transform: Transform,
    pub visibility: Visibility,
}

pub fn board(screen: ScreenState) -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    row_t(
        (Board, screen),
        board_size,
        (),
        fragment4(board_lines, board_numbers, wheel(screen), mistake_borders()),
    )
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

fn board_lines(props: &Props, spawner: &mut ChildSpawnerCommands) {
    use Orientation::*;
    use Thickness::*;

    spawner.spawn(line(props, 0, Horizontal, Thick));
    spawner.spawn(line(props, 1, Horizontal, Thin));
    spawner.spawn(line(props, 2, Horizontal, Thin));
    spawner.spawn(line(props, 3, Horizontal, Medium));
    spawner.spawn(line(props, 4, Horizontal, Thin));
    spawner.spawn(line(props, 5, Horizontal, Thin));
    spawner.spawn(line(props, 6, Horizontal, Medium));
    spawner.spawn(line(props, 7, Horizontal, Thin));
    spawner.spawn(line(props, 8, Horizontal, Thin));
    spawner.spawn(line(props, 9, Horizontal, Thick));
    spawner.spawn(line(props, 0, Vertical, Thick));
    spawner.spawn(line(props, 1, Vertical, Thin));
    spawner.spawn(line(props, 2, Vertical, Thin));
    spawner.spawn(line(props, 3, Vertical, Medium));
    spawner.spawn(line(props, 4, Vertical, Thin));
    spawner.spawn(line(props, 5, Vertical, Thin));
    spawner.spawn(line(props, 6, Vertical, Medium));
    spawner.spawn(line(props, 7, Vertical, Thin));
    spawner.spawn(line(props, 8, Vertical, Thin));
    spawner.spawn(line(props, 9, Vertical, Thick));
}

fn line(props: &Props, n: u8, orientation: Orientation, thickness: Thickness) -> impl Bundle {
    let theme = props.resources.theme;

    use Thickness::*;
    let (thickness, color, z) = match thickness {
        Thin => (0.03 * CELL_SIZE, theme.board_line_thin, 5.),
        Medium => (0.03 * CELL_SIZE, theme.board_line_medium, 6.),
        Thick => (0.06 * CELL_SIZE, theme.board_line_thick, 7.),
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

    (
        Sprite::from_color(color, Vec2::new(1., 1.)),
        Transform {
            translation,
            scale,
            ..default()
        },
    )
}

fn mistake_borders() -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    |props, spawner| {
        spawner
            .spawn(MistakeCellBordersBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 8.)),
                visibility: Visibility::Hidden,
                ..default()
            })
            .with_children(|spawner| {
                spawner.spawn_with_children(
                    props,
                    fragment4(
                        mistake_line(props, -0.5, Orientation::Horizontal),
                        mistake_line(props, 0.5, Orientation::Horizontal),
                        mistake_line(props, -0.5, Orientation::Vertical),
                        mistake_line(props, 0.5, Orientation::Vertical),
                    ),
                );
            });
    }
}

fn mistake_line(props: &Props, edge: f32, orientation: Orientation) -> impl Bundle {
    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., edge * CELL_SIZE, 1.),
        Vertical => Vec3::new(edge * CELL_SIZE, 0., 1.),
    };

    let scale = match orientation {
        Horizontal => Vec3::new(1.03 * CELL_SIZE, 0.06 * CELL_SIZE, 1.),
        Vertical => Vec3::new(0.06 * CELL_SIZE, 1.03 * CELL_SIZE, 1.),
    };

    (
        Sprite::from_color(props.resources.theme.mistake, Vec2::new(1., 1.)),
        Transform {
            translation,
            scale,
            ..default()
        },
    )
}
