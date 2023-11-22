use super::{board_numbers, wheel::wheel};
use crate::{constants::*, ui::*, utils::*, ScreenState};
use bevy::{prelude::*, sprite::SpriteBundle};

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

pub fn board(
    props: &Props,
    screen: ScreenState,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder)) {
    let bundle = FlexBundle::new(
        if props.resources.screen_sizing.is_ipad {
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
    );

    let spawn_children = move |props: &Props, cb: &mut ChildBuilder| {
        draw_lines(cb);

        cb.spawn_with_children(props, board_numbers);

        cb.spawn_with_children(props, wheel(screen));

        place_mistake_borders(cb);
    };

    ((Board, screen, bundle), spawn_children)
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

fn draw_lines(child_builder: &mut ChildBuilder) {
    use Orientation::*;
    use Thickness::*;

    child_builder.spawn(line(0, Horizontal, Thick));
    child_builder.spawn(line(1, Horizontal, Thin));
    child_builder.spawn(line(2, Horizontal, Thin));
    child_builder.spawn(line(3, Horizontal, Medium));
    child_builder.spawn(line(4, Horizontal, Thin));
    child_builder.spawn(line(5, Horizontal, Thin));
    child_builder.spawn(line(6, Horizontal, Medium));
    child_builder.spawn(line(7, Horizontal, Thin));
    child_builder.spawn(line(8, Horizontal, Thin));
    child_builder.spawn(line(9, Horizontal, Thick));
    child_builder.spawn(line(0, Vertical, Thick));
    child_builder.spawn(line(1, Vertical, Thin));
    child_builder.spawn(line(2, Vertical, Thin));
    child_builder.spawn(line(3, Vertical, Medium));
    child_builder.spawn(line(4, Vertical, Thin));
    child_builder.spawn(line(5, Vertical, Thin));
    child_builder.spawn(line(6, Vertical, Medium));
    child_builder.spawn(line(7, Vertical, Thin));
    child_builder.spawn(line(8, Vertical, Thin));
    child_builder.spawn(line(9, Vertical, Thick));
}

fn line(n: u8, orientation: Orientation, thickness: Thickness) -> impl Bundle {
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

fn place_mistake_borders(child_builder: &mut ChildBuilder) {
    child_builder
        .spawn(MistakeCellBordersBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 8.)),
            visibility: Visibility::Hidden,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(mistake_line(-0.5, Orientation::Horizontal));
            parent.spawn(mistake_line(0.5, Orientation::Horizontal));
            parent.spawn(mistake_line(-0.5, Orientation::Vertical));
            parent.spawn(mistake_line(0.5, Orientation::Vertical));
        });
}

fn mistake_line(edge: f32, orientation: Orientation) -> impl Bundle {
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
