use crate::ResourceBag;
use bevy::asset::Handle;
use bevy::ecs::world::Mut;
use bevy::render::texture::Image;

pub fn launch_screen(mut handle: Mut<'_, Handle<Image>>, resources: &ResourceBag) -> (f32, f32) {
    let image = if resources.screen_sizing.is_tablet() {
        &resources.images.launch_screen_ipad
    } else {
        &resources.images.launch_screen
    };

    if image.handle != *handle {
        *handle = image.handle.clone();
    }

    (image.width, image.height)
}

pub fn wall_image(mut handle: Mut<'_, Handle<Image>>, resources: &ResourceBag) -> (f32, f32) {
    let image = if resources.screen_sizing.is_tablet() {
        &resources.images.wall_ipad
    } else {
        &resources.images.wall
    };

    if image.handle != *handle {
        *handle = image.handle.clone();
    }

    (image.width, image.height)
}
