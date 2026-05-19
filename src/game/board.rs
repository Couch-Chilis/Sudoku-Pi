use bevy::prelude::*;

use crate::{constants::*, themes::Theme, ui::*, ScreenState};

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

#[derive(Component)]
pub enum LineThickness {
    Thin,
    Medium,
    Thick,
}

fn board_lines(_props: &Props, spawner: &mut ChildSpawnerCommands) {
    use LineThickness::*;
    use Orientation::*;

    spawner.spawn(line(0, Horizontal, Thick));
    spawner.spawn(line(1, Horizontal, Thin));
    spawner.spawn(line(2, Horizontal, Thin));
    spawner.spawn(line(3, Horizontal, Medium));
    spawner.spawn(line(4, Horizontal, Thin));
    spawner.spawn(line(5, Horizontal, Thin));
    spawner.spawn(line(6, Horizontal, Medium));
    spawner.spawn(line(7, Horizontal, Thin));
    spawner.spawn(line(8, Horizontal, Thin));
    spawner.spawn(line(9, Horizontal, Thick));
    spawner.spawn(line(0, Vertical, Thick));
    spawner.spawn(line(1, Vertical, Thin));
    spawner.spawn(line(2, Vertical, Thin));
    spawner.spawn(line(3, Vertical, Medium));
    spawner.spawn(line(4, Vertical, Thin));
    spawner.spawn(line(5, Vertical, Thin));
    spawner.spawn(line(6, Vertical, Medium));
    spawner.spawn(line(7, Vertical, Thin));
    spawner.spawn(line(8, Vertical, Thin));
    spawner.spawn(line(9, Vertical, Thick));
}

fn line(n: u8, orientation: Orientation, thickness: LineThickness) -> impl Bundle {
    use LineThickness::*;
    let (width, z) = match thickness {
        Thin => (0.03 * CELL_SIZE, 5.),
        Medium => (0.03 * CELL_SIZE, 6.),
        Thick => (0.06 * CELL_SIZE, 7.),
    };

    use Orientation::*;
    let translation = match orientation {
        Horizontal => Vec3::new(0., (n as f32 - 4.5) * CELL_SIZE, z),
        Vertical => Vec3::new((n as f32 - 4.5) * CELL_SIZE, 0., z),
    };

    let scale = match orientation {
        Horizontal => Vec3::new(9.03 * CELL_SIZE, width, 1.),
        Vertical => Vec3::new(width, 9.03 * CELL_SIZE, 1.),
    };

    (
        thickness,
        Sprite::default(),
        Transform {
            translation,
            scale,
            ..default()
        },
    )
}

pub fn render_board_lines(
    mut lines_query: Query<(&mut Sprite, &LineThickness)>,
    theme: Res<Theme>,
) {
    if !theme.is_changed() {
        return;
    }

    for (mut sprite, thickness) in &mut lines_query {
        use LineThickness::*;
        let color = match thickness {
            Thin => theme.board_line_thin,
            Medium => theme.board_line_medium,
            Thick => theme.board_line_thick,
        };

        *sprite = Sprite::from_color(color, Vec2::new(1., 1.));
    }
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
