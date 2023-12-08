use super::flex::*;
use crate::constants::*;
use crate::{ResourceBag, ScreenState};
use bevy::prelude::*;
use std::sync::Arc;

pub fn align_self(align_self: Alignment) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.align_self = align_self;
    }
}

pub fn alignment(alignment: TextAlignment) -> impl FnOnce(&mut Text, &ResourceBag) {
    move |text: &mut Text, _resources: &ResourceBag| {
        text.alignment = alignment;
    }
}

pub fn available_size(style: &mut FlexItemStyle) {
    style.flex_grow = 1.;
}

pub fn board_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            style.flex_shrink = 1.;
            style.min_size = Size::all(Val::Vmin(50.));
            style.preserve_aspect_ratio = true;

            style.flex_base = if resources.screen_sizing.is_tablet() {
                Size::all(Val::Vmin(90.))
            } else {
                Size::all(Val::Vmin(80.))
            };
        },
    ))
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

pub fn button_size_main(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let ratio = resources.screen_sizing.portrait_ratio();
            let base = 10. * ratio.clamp(0.5, 0.8);
            let ratio = 10. * ratio.clamp(0.7, 1.);

            style.flex_base = Size::new(Val::Vmin(ratio * base), Val::Vmin(base));
        },
    ));
}

pub fn button_size_settings(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            style.flex_base = if resources.screen_sizing.is_tablet() {
                Size::new(Val::Pixel(600), Val::Pixel(60))
            } else {
                Size::new(Val::Vmin(50.), Val::Vmin(10.))
            };
        },
    ));
}

pub fn button_text(text: &mut Text, resources: &ResourceBag) {
    button_text_color(text, resources);
    button_text_font(text, resources);
    button_text_size(text, resources);
}

pub fn button_text_color(text: &mut Text, _resources: &ResourceBag) {
    text.sections[0].style.color = COLOR_BUTTON_TEXT;
}

pub fn button_text_font(text: &mut Text, resources: &ResourceBag) {
    text.sections[0].style.font = resources.fonts.medium.clone();
}

pub fn button_text_size(text: &mut Text, resources: &ResourceBag) {
    text.sections[0].style.font_size = if resources.screen_sizing.is_tablet() {
        66.
    } else {
        44.
    };
}

pub fn cog_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let cog_size = if resources.screen_sizing.is_ipad {
                Val::Pixel(40)
            } else {
                Val::Pixel(30)
            };

            fixed_size(cog_size.clone(), cog_size)(style);
        },
    ));
}

pub fn fixed_aspect_ratio(style: &mut FlexItemStyle) {
    style.preserve_aspect_ratio = true;
}

pub fn font_bold(text: &mut Text, resources: &ResourceBag) {
    text.sections[0].style.font = resources.fonts.bold.clone();
}

pub fn font_medium(text: &mut Text, resources: &ResourceBag) {
    text.sections[0].style.font = resources.fonts.medium.clone();
}

pub fn fixed_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
    }
}

pub fn font_size(font_size: f32) -> impl FnOnce(&mut Text, &ResourceBag) {
    move |text: &mut Text, _resources: &ResourceBag| {
        text.sections[0].style.font_size = font_size;
    }
}

pub fn margin(margin: Size) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.margin = margin;
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

pub fn screen_gap(screen: ScreenState) -> impl FnOnce(&mut FlexContainerStyle) {
    move |style: &mut FlexContainerStyle| {
        style.gap = if screen == ScreenState::Game {
            Val::Auto
        } else {
            Val::None
        }
    }
}

pub fn screen_padding(
    resources: &ResourceBag,
    screen: ScreenState,
) -> impl FnOnce(&mut FlexContainerStyle) {
    let is_tablet = resources.screen_sizing.is_tablet();
    let top_padding = resources.screen_sizing.top_padding;

    move |style: &mut FlexContainerStyle| {
        style.padding = Sides {
            top: if screen == ScreenState::MainMenu {
                Val::None
            } else if is_tablet && screen == ScreenState::Game {
                Val::Auto
            } else {
                Val::Pixel(top_padding)
            },
            right: Val::None,
            bottom: if is_tablet && screen == ScreenState::Game {
                Val::Auto
            } else {
                Val::None
            },
            left: Val::None,
        }
    }
}

pub fn text_color(color: Color) -> impl FnOnce(&mut Text, &ResourceBag) {
    move |text: &mut Text, _resources: &ResourceBag| {
        text.sections[0].style.color = color;
    }
}

pub fn transform(transform: Transform) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.transform = transform;
    }
}

pub fn translation(translation: Vec3) -> impl FnOnce(&mut FlexItemStyle) {
    transform(Transform::from_translation(translation))
}

pub fn without_occupying_space(style: &mut FlexItemStyle) {
    style.occupies_space = false;
}
