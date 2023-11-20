use super::flex::*;
use crate::constants::COLOR_BUTTON_TEXT;
use crate::resource_bag::ResourceBag;
use bevy::text::TextStyle;

pub trait FlexItemStyleEnhancer: Sized {
    fn enhance(self, style: &mut FlexItemStyle);
}

impl<T> FlexItemStyleEnhancer for T
where
    T: FnOnce(&mut FlexItemStyle) + Sized,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self(style)
    }
}

impl FlexItemStyleEnhancer for () {
    fn enhance(self, _style: &mut FlexItemStyle) {}
}

impl<T, U> FlexItemStyleEnhancer for (T, U)
where
    T: FlexItemStyleEnhancer,
    U: FlexItemStyleEnhancer,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
    }
}

pub fn available_size(style: &mut FlexItemStyle) {
    style.flex_grow = 1.;
}

pub fn button_margin(style: &mut FlexItemStyle) {
    style.margin = Size::all(Val::Vmin(1.5));
}

pub fn button_margin_extra_height(style: &mut FlexItemStyle) {
    style.margin = Size::new(Val::Vmin(1.5), Val::Vmin(5.))
}

pub fn button_margin_extra_height_on_ios(style: &mut FlexItemStyle) {
    if cfg!(target_os = "ios") {
        button_margin_extra_height(style)
    } else {
        button_margin(style)
    };
}

pub fn button_size_main(resources: &ResourceBag) -> impl FnOnce(&mut FlexItemStyle) {
    let flex_base = if resources.screen_sizing.is_ipad {
        Size::new(Val::Pixel(600), Val::Pixel(60))
    } else {
        Size::new(Val::Vmin(70.), Val::Vmin(10.))
    };

    move |style: &mut FlexItemStyle| {
        style.flex_base = flex_base;
    }
}

pub fn button_text(resources: &ResourceBag) -> impl FnOnce(&mut TextStyle) {
    let font = resources.fonts.medium.clone();
    let font_size = if resources.screen_sizing.is_ipad {
        66.
    } else {
        44.
    };

    move |style: &mut TextStyle| {
        style.color = COLOR_BUTTON_TEXT;
        style.font = font;
        style.font_size = font_size;
    }
}
