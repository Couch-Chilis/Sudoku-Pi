use crate::{utils::vmin_size, WindowSize};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct LogoBundle {
    image: ImageBundle,
}

impl LogoBundle {
    pub fn new(asset_server: &AssetServer, window_size: &WindowSize) -> Self {
        let image = ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(7.0 * window_size.vmin_scale),
                    top: Val::Px(7.0 * window_size.vmin_scale),
                    ..default()
                },
                size: vmin_size(&window_size, 35.0, 75.0),
                ..default()
            },
            image: UiImage {
                texture: asset_server.load("logo.png"),
                ..default()
            },
            ..default()
        };

        Self { image }
    }
}
