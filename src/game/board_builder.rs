use super::OnGameScreen;
use crate::constants::CELL_SIZE;
use crate::WindowSize;
use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub struct Board;

pub fn build_board<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    window_size: &WindowSize,
) -> EntityCommands<'w, 's, 'a> {
    let scale = 90. * window_size.vmin_scale;

    let mut board = commands.spawn((
        Board,
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(scale, scale, 1.),
                translation: Vec3::new(0., 0., 2.),
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    ));

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

    board
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub enum Thickness {
    Thin,
    Medium,
    Thick,
}

pub fn build_line(n: u8, orientation: Orientation, thickness: Thickness) -> impl Bundle {
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
        sprite: Sprite {
            color: Color::BLACK,
            ..default()
        },
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        ..default()
    }
}
