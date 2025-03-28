use bevy::prelude::*;

use crate::ResourceBag;

pub fn launch_screen(mut sprite: Mut<Sprite>, resources: &ResourceBag) -> (f32, f32) {
    let image = if resources.screen_sizing.is_tablet() {
        &resources.images.launch_screen_ipad
    } else {
        &resources.images.launch_screen
    };

    if sprite.image != image.handle {
        sprite.image = image.handle.clone();
    }

    (image.width, image.height)
}

pub fn wall_image(mut sprite: Mut<Sprite>, resources: &ResourceBag) -> (f32, f32) {
    let image = if resources.screen_sizing.is_tablet() {
        &resources.images.wall_ipad
    } else {
        &resources.images.wall
    };

    if sprite.image != image.handle {
        sprite.image = image.handle.clone();
    }

    (image.width, image.height)
}
