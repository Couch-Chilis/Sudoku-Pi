use crate::WindowSize;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct SelectedNumberBundle {
    sprite: SpriteBundle,
}

impl SelectedNumberBundle {
    pub fn new(window_size: &WindowSize, x: u8, y: u8) -> Self {
        let scale = 10. * window_size.vmin_scale;

        let scale = match orientation {
            Horizontal => Vec3::new(9.075 * scale, thickness, 1.),
            Vertical => Vec3::new(thickness, 9.075 * scale, 1.),
        };

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                ..default()
            },
            transform: Transform {
                translation,
                scale,
                ..default()
            },
            ..default()
        };

        Self { sprite }
    }
}
