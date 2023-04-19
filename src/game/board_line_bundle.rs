use crate::WindowSize;
use bevy::prelude::*;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub enum Thickness {
    Thin,
    Medium,
    Thick,
}

#[derive(Bundle)]
pub struct BoardLineBundle {
    sprite: SpriteBundle,
}

impl BoardLineBundle {
    pub fn new(
        window_size: &WindowSize,
        n: u8,
        orientation: Orientation,
        thickness: Thickness,
    ) -> Self {
        let scale = 10. * window_size.vmin_scale;

        use Orientation::*;
        let translation = match orientation {
            Horizontal => Vec3::new(0., (n as f32 - 4.5) * scale, 1.),
            Vertical => Vec3::new((n as f32 - 4.5) * scale, 0., 1.),
        };

        use Thickness::*;
        let thickness = match thickness {
            Thin => 0.05 * scale,
            Medium => 0.1 * scale,
            Thick => 0.15 * scale,
        };

        let scale = match orientation {
            Horizontal => Vec3::new(9. * scale, thickness, 1.),
            Vertical => Vec3::new(thickness, 9. * scale, 1.),
        };

        let sprite = SpriteBundle {
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
        };

        Self { sprite }
    }
}
