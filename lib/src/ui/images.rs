use crate::assets::*;
use crate::ResourceBag;

pub fn launch_screen(resources: &ResourceBag) -> ImageWithDimensions {
    if resources.screen_sizing.is_tablet() {
        resources.images.launch_screen_ipad.clone()
    } else {
        resources.images.launch_screen.clone()
    }
}

pub fn wall_image(resources: &ResourceBag) -> ImageWithDimensions {
    if resources.screen_sizing.is_tablet() {
        resources.images.wall_ipad.clone()
    } else {
        resources.images.wall.clone()
    }
}
