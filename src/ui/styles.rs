use std::sync::Arc;

use bevy::sprite::Anchor;
use bevy::{prelude::*, text::TextBounds};

use crate::{ResourceBag, Theme};

use super::flex::*;

const GAME_SCREEN_TIMER_LINE_HEIGHT: Val = Val::Pixel(1);

pub fn align_self(align_self: Alignment) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.align_self = align_self;
    }
}

pub fn available_size(style: &mut FlexItemStyle) {
    style.flex_grow = 1.;
}

pub fn board_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            style.flex_shrink = 1.;
            style.min_size = Size::all(Val::Vmin(50.));
            style.preserve_aspect_ratio = true;

            style.flex_base = if bag.screen_sizing.is_tablet() {
                Size::all(Val::Vmin(90.))
            } else {
                Size::all(Val::Vmin(80.))
            };
        }))
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
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let ratio = bag.screen_sizing.portrait_ratio();
            let base = 10. * ratio.clamp(0.5, 0.8);
            let ratio = 10. * ratio.clamp(0.7, 1.);

            style.flex_base = Size::new(Val::Vmin(ratio * base), Val::Vmin(base));
        }));
}

pub fn button_size_settings(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            style.flex_base = if bag.screen_sizing.is_tablet() {
                Size::new(Val::Pixel(600), Val::Pixel(60))
            } else {
                Size::new(Val::Vmin(50.), Val::Vmin(10.))
            };
        }));
}

pub fn button_text(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    button_text_color(bundle, bag);
    button_text_font(bundle, bag);
    button_text_size(bundle, bag);
}

pub fn button_text_color(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.color = bag.theme.button_text.into();
}

pub fn button_text_font(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font = bag.fonts.medium.clone().into();
}

pub fn button_text_size(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font_size = if bag.screen_sizing.is_tablet() {
        55.
    } else {
        36.7
    }
    .into();
}

pub fn cog_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let cog_size = if bag.screen_sizing.is_tablet() {
                Val::Pixel(40)
            } else {
                Val::Pixel(30)
            };

            fixed_size(cog_size.clone(), cog_size)(style);
        }));
}

pub fn fixed_aspect_ratio(style: &mut FlexItemStyle) {
    style.preserve_aspect_ratio = true;
}

pub fn font_bold(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font = bag.fonts.bold.clone().into();
}

pub fn font_medium(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font = bag.fonts.medium.clone().into();
}

pub fn fixed_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
    }
}

pub fn font_size(font_size: f32) -> impl FnOnce(&mut FlexTextBundle, &ResourceBag) {
    move |bundle: &mut FlexTextBundle, _bag: &ResourceBag| {
        bundle.font.font_size = font_size.into();
    }
}

pub fn game_screen_score_font_size(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font_size = if bag.screen_sizing.is_tablet() {
        71.7
    } else {
        48.3
    }
    .into();
}

pub fn game_screen_score_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let (width, height) = if bag.screen_sizing.is_tablet() {
                (Val::Pixel(150), Val::Pixel(60))
            } else {
                (Val::Pixel(100), Val::Pixel(35))
            };
            fixed_size(width, height)(style)
        }));
}

pub fn game_screen_timer_font_size(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.font.font_size = if bag.screen_sizing.is_tablet() {
        87.5
    } else {
        58.3
    }
    .into();
}

pub fn game_screen_timer_inner_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            minimum_size(
                get_game_screen_timer_width(bag),
                get_game_screen_timer_height(bag) - 2. * GAME_SCREEN_TIMER_LINE_HEIGHT,
            )(style)
        }));
}

pub fn game_screen_timer_line_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            fixed_size(
                get_game_screen_timer_width(bag),
                GAME_SCREEN_TIMER_LINE_HEIGHT,
            )(style)
        }));
}

pub fn game_screen_top_row_button_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let (width, height) = if bag.screen_sizing.is_tablet() {
                (Val::Pixel(133), Val::Pixel(60))
            } else {
                (Val::Pixel(80), Val::Pixel(35))
            };
            fixed_size(width, height)(style)
        }));
}

pub fn game_screen_top_row_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let width = Val::Vmin(if bag.screen_sizing.is_tablet() {
                80.
            } else {
                90.
            });
            let height = Val::Pixel(35);
            preferred_size(width, height)(style)
        }));
}

pub fn gap(gap: Val) -> impl FnOnce(&mut FlexContainerBundle, &ResourceBag) {
    move |bundle: &mut FlexContainerBundle, _bag: &ResourceBag| {
        bundle.style.gap = gap;
    }
}

pub fn highscore_screen_button_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let (width, height) = if bag.screen_sizing.is_tablet() {
                (Val::Vmin(60.), Val::Vmin(6.))
            } else {
                (Val::Vmin(70.), Val::Vmin(10.))
            };
            fixed_size(width, height)(style)
        }));
}

pub fn highscore_screen_wall_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            fixed_size(
                Val::Percent(100.),
                Val::CrossPercent(if bag.screen_sizing.is_tablet() {
                    59.8
                } else {
                    102.5
                }),
            )(style)
        }));
}

pub fn highscore_scroll_author_padding(bundle: &mut FlexContainerBundle, _bag: &ResourceBag) {
    bundle.style.dynamic_styles.push(Arc::new(
        |style: &mut FlexContainerStyle, bag: &ResourceBag| {
            apply_highscore_scroll_padding(style, bag);

            if bag.screen_sizing.is_tablet() {
                style.padding.top = Val::Pixel(155);
                style.padding.right += Val::Pixel(15);
            } else {
                style.padding.top = Val::Pixel(65);
                style.padding.right += Val::Pixel(10);
            }
        },
    ));
}

pub fn highscore_scroll_padding(bundle: &mut FlexContainerBundle, _bag: &ResourceBag) {
    bundle
        .style
        .dynamic_styles
        .push(Arc::new(apply_highscore_scroll_padding));
}

pub fn highscore_scroll_quote(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.anchor = Anchor::CENTER;
    bundle.color = Color::BLACK.into();
    bundle.font.font = bag.fonts.scroll.clone().into();

    if bag.screen_sizing.is_tablet() {
        bundle.font.font_size = FontSize::from(50.);
        bundle.bounds = TextBounds {
            width: Some(1200.),
            height: None,
        };
    } else {
        bundle.font.font_size = FontSize::from(30.);
        bundle.bounds = TextBounds {
            width: Some(580.),
            height: None,
        };
    }
}

pub fn highscore_scroll_author(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.anchor = Anchor::BOTTOM_RIGHT;
    bundle.color = Color::BLACK.into();
    bundle.font.font = bag.fonts.scroll.clone().into();
    bundle.font.font_size = if bag.screen_sizing.is_tablet() {
        41.7
    } else {
        25.
    }
    .into();
    bundle.layout.justify = Justify::Right;
}

pub fn highscore_scroll_size(style: &mut FlexItemStyle) {
    style
        .dynamic_styles
        .push(Arc::new(|style: &mut FlexItemStyle, bag: &ResourceBag| {
            let (width, height) = if bag.screen_sizing.is_tablet() {
                (Val::Vmin(70.), Val::Vmin(19.))
            } else {
                (Val::Pixel(342), Val::Pixel(92))
            };

            fixed_size(width, height)(style);
        }));
}

pub fn justify(justify: Justify) -> impl FnOnce(&mut FlexTextBundle, &ResourceBag) {
    move |bundle: &mut FlexTextBundle, _bag: &ResourceBag| {
        bundle.layout.justify = justify;
    }
}

pub fn margin(margin: Size) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.margin = margin;
    }
}

pub fn minimum_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
        style.flex_grow = 1.;
    }
}

pub fn padding(padding: Sides) -> impl FnOnce(&mut FlexContainerBundle, &ResourceBag) {
    move |bundle: &mut FlexContainerBundle, _bag: &ResourceBag| {
        bundle.style.padding = padding;
    }
}

pub fn preferred_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
        style.flex_shrink = 1.;
    }
}

pub fn score_board_padding(bundle: &mut FlexContainerBundle, _bag: &ResourceBag) {
    bundle.style.dynamic_styles.push(Arc::new(|style, bag| {
        style.padding = if bag.screen_sizing.is_tablet() {
            Sides {
                top: Val::Percent(32.),
                right: Val::Percent(27.),
                bottom: Val::Percent(12.),
                left: Val::Percent(27.),
            }
        } else {
            Sides {
                top: Val::Percent(30.),
                right: Val::Percent(15.),
                bottom: Val::Percent(10.),
                left: Val::Percent(15.),
            }
        };
    }));
}

pub fn settings_label_text(bundle: &mut FlexTextBundle, bag: &ResourceBag) {
    bundle.color = bag.theme.secondary_button_text.into();
    bundle.font.font = bag.fonts.medium.clone().into();
    bundle.font.font_size = if bag.screen_sizing.is_tablet() {
        60.
    } else {
        41.7
    }
    .into();
}

pub fn text_anchor(anchor: Anchor) -> impl FnOnce(&mut FlexTextBundle, &ResourceBag) {
    move |bundle: &mut FlexTextBundle, _bag: &ResourceBag| {
        bundle.anchor = anchor;
    }
}

pub fn theme_background_color(
    selector: impl FnOnce(&Theme) -> Color,
) -> impl FnOnce(&mut FlexContainerBundle, &ResourceBag) {
    move |bundle: &mut FlexContainerBundle, bag: &ResourceBag| {
        bundle.sprite = Sprite::from_color(selector(&bag.theme), Vec2::new(1., 1.));
    }
}

pub fn theme_text_color(
    selector: impl FnOnce(&Theme) -> Color,
) -> impl FnOnce(&mut FlexTextBundle, &ResourceBag) {
    move |bundle: &mut FlexTextBundle, bag: &ResourceBag| {
        bundle.color = selector(&bag.theme).into();
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

pub fn z_index(z_index: f32) -> impl FnOnce(&mut FlexItemStyle) {
    translation(Vec3::new(0., 0., z_index))
}

fn apply_highscore_scroll_padding(style: &mut FlexContainerStyle, bag: &ResourceBag) {
    style.padding = if bag.screen_sizing.is_tablet() {
        Sides::new(Val::Vmin(3.), Val::Vmin(2.2))
    } else {
        Sides::new(Val::Pixel(16), Val::Pixel(10))
    };
}

fn get_game_screen_timer_height(bag: &ResourceBag) -> Val {
    if bag.screen_sizing.is_tablet() {
        Val::Pixel(64)
    } else {
        Val::Pixel(42)
    }
}

fn get_game_screen_timer_width(bag: &ResourceBag) -> Val {
    if bag.screen_sizing.is_tablet() {
        Val::Pixel(150)
    } else {
        Val::Pixel(100)
    }
}
