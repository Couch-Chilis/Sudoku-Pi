use bevy::prelude::*;

use crate::ResourceBag;

pub fn launch_screen(mut sprite: Mut<Sprite>, bag: &ResourceBag) -> (f32, f32) {
    let image = match (bag.screen_sizing.is_tablet(), bag.theme.is_dark_mode()) {
        (false, false) => &bag.images.launch_screen,
        (false, true) => &bag.images.launch_screen_dark,
        (true, false) => &bag.images.launch_screen_ipad,
        (true, true) => &bag.images.launch_screen_ipad_dark,
    };

    if sprite.image != image.handle {
        sprite.image = image.handle.clone();
    }

    (image.width, image.height)
}

pub fn wall_image(mut sprite: Mut<Sprite>, bag: &ResourceBag) -> (f32, f32) {
    let image = match (bag.screen_sizing.is_tablet(), bag.theme.is_dark_mode()) {
        (false, false) => &bag.images.wall,
        (false, true) => &bag.images.wall_dark,
        (true, false) => &bag.images.wall_ipad,
        (true, true) => &bag.images.wall_ipad_dark,
    };

    if sprite.image != image.handle {
        sprite.image = image.handle.clone();
    }

    (image.width, image.height)
}
