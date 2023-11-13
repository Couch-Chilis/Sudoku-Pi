use crate::assets::{Fonts, Images};
use crate::ScreenSizing;
use bevy::prelude::*;

pub type ResourceTuple<'a> = (Res<'a, Fonts>, Res<'a, Images>, Res<'a, ScreenSizing>);

/// Bag of resources we need to commonly access, especially for UI construction.
pub struct ResourceBag<'a> {
    pub fonts: &'a Fonts,
    pub images: &'a Images,
    pub screen_sizing: &'a ScreenSizing,
}

impl<'a> ResourceBag<'a> {
    pub fn from_tuple((fonts, images, screen_sizing): &'a ResourceTuple<'a>) -> Self {
        Self {
            fonts,
            images,
            screen_sizing,
        }
    }
}
