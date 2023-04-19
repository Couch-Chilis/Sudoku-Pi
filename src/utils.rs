use crate::WindowSize;
use bevy::prelude::*;

pub fn vmin_size(window_size: &WindowSize, width: f32, height: f32) -> Size {
    Size::new(
        Val::Px(width * window_size.vmin_scale),
        Val::Px(height * window_size.vmin_scale),
    )
}
