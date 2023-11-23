use super::flex::*;
use crate::constants::COLOR_BUTTON_TEXT;
use crate::resource_bag::ResourceBag;
use bevy::prelude::*;

pub fn align_self(align_self: Alignment) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.align_self = align_self;
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

pub fn button_size_onboarding(resources: &ResourceBag) -> impl FnOnce(&mut FlexItemStyle) {
    let flex_base = if resources.screen_sizing.is_ipad {
        Size::new(Val::Pixel(600), Val::Pixel(60))
    } else {
        Size::new(Val::Vmin(50.), Val::Vmin(10.))
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

pub fn font_bold(resources: &ResourceBag) -> impl FnOnce(&mut TextStyle) {
    let font = resources.fonts.bold.clone();

    move |style: &mut TextStyle| {
        style.font = font;
    }
}

pub fn font_medium(resources: &ResourceBag) -> impl FnOnce(&mut TextStyle) {
    let font = resources.fonts.medium.clone();

    move |style: &mut TextStyle| {
        style.font = font;
    }
}

pub fn fixed_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
    }
}

pub fn font_size(font_size: f32) -> impl FnOnce(&mut TextStyle) {
    move |style: &mut TextStyle| {
        style.font_size = font_size;
    }
}

pub fn padding(padding: Sides) -> impl FnOnce(&mut FlexContainerStyle) {
    move |style: &mut FlexContainerStyle| {
        style.padding = padding;
    }
}

pub fn preferred_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
        style.flex_shrink = 1.;
    }
}

pub fn text_color(color: Color) -> impl FnOnce(&mut TextStyle) {
    move |style: &mut TextStyle| {
        style.color = color;
    }
}

pub fn translation(translation: Vec3) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.transform = Transform::from_translation(translation);
    }
}

pub fn without_occupying_space(style: &mut FlexItemStyle) {
    style.occupies_space = false;
}
