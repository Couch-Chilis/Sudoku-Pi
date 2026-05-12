use crate::assets::{Fonts, Images};
use crate::{ScreenSizing, Theme};
use bevy::prelude::*;

pub type ResourceTuple<'w> = (
    Res<'w, Fonts>,
    Res<'w, Images>,
    Res<'w, ScreenSizing>,
    Res<'w, Theme>,
);

/// Bag of resources we need to commonly access, especially for UI construction.
pub struct ResourceBag<'w> {
    pub fonts: &'w Fonts,
    pub images: &'w Images,
    pub screen_sizing: &'w ScreenSizing,
    pub theme: &'w Theme,
}

impl<'w> ResourceBag<'w> {
    pub fn from_tuple((fonts, images, screen_sizing, theme): &'w ResourceTuple<'w>) -> Self {
        Self {
            fonts,
            images,
            screen_sizing,
            theme,
        }
    }
}
