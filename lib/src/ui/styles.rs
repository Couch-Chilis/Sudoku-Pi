use super::flex::*;
use crate::constants::*;
use crate::utils::SpriteExt;
use crate::{ResourceBag, ScreenState};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;
use std::sync::Arc;

const GAME_SCREEN_TIMER_LINE_HEIGHT: Val = Val::Pixel(1);

pub fn align_self(align_self: Alignment) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.align_self = align_self;
    }
}

pub fn available_size(style: &mut FlexItemStyle) {
    style.flex_grow = 1.;
}

pub fn background_color(color: Color) -> impl FnOnce(&mut FlexContainerBundle) {
    move |bundle: &mut FlexContainerBundle| {
        bundle.background = Sprite::from_color(color);
    }
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

pub fn button_text(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    button_text_color(bundle, resources);
    button_text_font(bundle, resources);
    button_text_size(bundle, resources);
}

pub fn button_text_color(bundle: &mut Text2dBundle, _resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style.color = COLOR_BUTTON_TEXT;
    }
}

pub fn button_text_font(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style.font = resources.fonts.medium.clone();
    }
}

pub fn button_text_size(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style.font_size = if resources.screen_sizing.is_tablet() {
            66.
        } else {
            44.
        };
    }
}

pub fn cog_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let cog_size = if resources.screen_sizing.is_tablet() {
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

pub fn font_bold(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style.font = resources.fonts.bold.clone();
    }
}

pub fn font_medium(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style.font = resources.fonts.medium.clone();
    }
}

pub fn fixed_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
    }
}

pub fn font_size(font_size: f32) -> impl FnOnce(&mut Text2dBundle, &ResourceBag) {
    move |text: &mut Text2dBundle, _resources: &ResourceBag| {
        for section in &mut text.text.sections {
            section.style.font_size = font_size;
        }
    }
}

pub fn game_screen_score_font_size(text: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut text.text.sections {
        section.style.font_size = if resources.screen_sizing.is_tablet() {
            86.
        } else {
            58.
        };
    }
}

pub fn game_screen_score_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let (width, height) = if resources.screen_sizing.is_tablet() {
                (Val::Pixel(150), Val::Pixel(60))
            } else {
                (Val::Pixel(100), Val::Pixel(35))
            };
            fixed_size(width, height)(style)
        },
    ));
}

pub fn game_screen_timer_font_size(text: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut text.text.sections {
        section.style.font_size = if resources.screen_sizing.is_tablet() {
            105.
        } else {
            70.
        }
    }
}

pub fn game_screen_timer_inner_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            minimum_size(
                get_game_screen_timer_width(resources),
                get_game_screen_timer_height(resources) - 2. * GAME_SCREEN_TIMER_LINE_HEIGHT,
            )(style)
        },
    ));
}

pub fn game_screen_timer_line_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            fixed_size(
                get_game_screen_timer_width(resources),
                GAME_SCREEN_TIMER_LINE_HEIGHT,
            )(style)
        },
    ));
}

pub fn game_screen_top_row_button_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let (width, height) = if resources.screen_sizing.is_tablet() {
                (Val::Pixel(133), Val::Pixel(60))
            } else {
                (Val::Pixel(80), Val::Pixel(35))
            };
            fixed_size(width, height)(style)
        },
    ));
}

pub fn game_screen_top_row_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let width = Val::Vmin(if resources.screen_sizing.is_tablet() {
                80.
            } else {
                90.
            });
            let height = Val::Pixel(35);
            preferred_size(width, height)(style)
        },
    ));
}

pub fn gap(gap: Val) -> impl FnOnce(&mut FlexContainerBundle) {
    move |bundle: &mut FlexContainerBundle| {
        bundle.style.gap = gap;
    }
}

pub fn highscore_screen_button_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let (width, height) = if resources.screen_sizing.is_tablet() {
                (Val::Pixel(600), Val::Pixel(60))
            } else {
                (Val::Vmin(70.), Val::Vmin(10.))
            };
            fixed_size(width, height)(style)
        },
    ));
}

pub fn highscore_screen_wall_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            fixed_size(
                Val::Percent(100.),
                Val::CrossPercent(if resources.screen_sizing.is_tablet() {
                    59.8
                } else {
                    102.5
                }),
            )(style)
        },
    ));
}

pub fn highscore_scroll_author_padding(bundle: &mut FlexContainerBundle) {
    bundle.style.dynamic_styles.push(Arc::new(
        |style: &mut FlexContainerStyle, resources: &ResourceBag| {
            apply_highscore_scroll_padding(style, resources);

            style.padding.top = Val::Pixel(if resources.screen_sizing.is_tablet() {
                155
            } else {
                65
            });
            style.padding.right += Val::Pixel(if resources.screen_sizing.is_tablet() {
                15
            } else {
                10
            });
        },
    ));
}

pub fn highscore_scroll_padding(bundle: &mut FlexContainerBundle) {
    bundle
        .style
        .dynamic_styles
        .push(Arc::new(apply_highscore_scroll_padding));
}

pub fn highscore_scroll_quote_text_bounds(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    bundle.text_2d_bounds = Text2dBounds {
        size: Vec2::new(
            if resources.screen_sizing.is_tablet() {
                1200.
            } else {
                580.
            },
            if resources.screen_sizing.is_tablet() {
                400.
            } else {
                200.
            },
        ),
    };
}

pub fn highscore_scroll_size(style: &mut FlexItemStyle) {
    style.dynamic_styles.push(Arc::new(
        |style: &mut FlexItemStyle, resources: &ResourceBag| {
            let (width, height) = if resources.screen_sizing.is_tablet() {
                (Val::Pixel(700), Val::Pixel(190))
            } else {
                (Val::Pixel(342), Val::Pixel(92))
            };

            fixed_size(width, height)(style);
        },
    ));
}

pub fn justify(justify: JustifyText) -> impl FnOnce(&mut Text2dBundle, &ResourceBag) {
    move |bundle: &mut Text2dBundle, _resources: &ResourceBag| {
        bundle.text.justify = justify;
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

pub fn padding(padding: Sides) -> impl FnOnce(&mut FlexContainerBundle) {
    move |bundle: &mut FlexContainerBundle| {
        bundle.style.padding = padding;
    }
}

pub fn preferred_size(width: Val, height: Val) -> impl FnOnce(&mut FlexItemStyle) {
    move |style: &mut FlexItemStyle| {
        style.flex_base = Size::new(width, height);
        style.flex_shrink = 1.;
    }
}

pub fn score_board_padding(bundle: &mut FlexContainerBundle) {
    bundle
        .style
        .dynamic_styles
        .push(Arc::new(|style, resources| {
            style.padding = if resources.screen_sizing.is_tablet() {
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

pub fn screen_gap(screen: ScreenState) -> impl FnOnce(&mut FlexContainerBundle) {
    move |bundle: &mut FlexContainerBundle| {
        bundle.style.gap = if screen == ScreenState::Game {
            Val::Auto
        } else {
            Val::None
        }
    }
}

pub fn screen_padding(
    resources: &ResourceBag,
    screen: ScreenState,
) -> impl FnOnce(&mut FlexContainerBundle) {
    let is_tablet = resources.screen_sizing.is_tablet();
    let top_padding = resources.screen_sizing.top_padding;

    move |bundle: &mut FlexContainerBundle| {
        bundle.style.padding = Sides {
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

pub fn settings_label_text(bundle: &mut Text2dBundle, resources: &ResourceBag) {
    for section in &mut bundle.text.sections {
        section.style = TextStyle {
            color: COLOR_SECONDARY_BUTTON_TEXT,
            font: resources.fonts.medium.clone(),
            font_size: if resources.screen_sizing.is_tablet() {
                72.
            } else {
                50.
            },
        }
    }
}

pub fn text_anchor(anchor: Anchor) -> impl FnOnce(&mut Text2dBundle, &ResourceBag) {
    move |bundle: &mut Text2dBundle, _resources: &ResourceBag| {
        bundle.text_anchor = anchor;
    }
}

pub fn text_color(color: Color) -> impl FnOnce(&mut Text2dBundle, &ResourceBag) {
    move |bundle: &mut Text2dBundle, _resources: &ResourceBag| {
        for section in &mut bundle.text.sections {
            section.style.color = color;
        }
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

fn apply_highscore_scroll_padding(style: &mut FlexContainerStyle, resources: &ResourceBag) {
    style.padding = if resources.screen_sizing.is_tablet() {
        Sides::new(Val::Pixel(30), Val::Pixel(22))
    } else {
        Sides::new(Val::Pixel(16), Val::Pixel(10))
    };
}

fn get_game_screen_timer_height(resources: &ResourceBag) -> Val {
    if resources.screen_sizing.is_tablet() {
        Val::Pixel(64)
    } else {
        Val::Pixel(42)
    }
}

fn get_game_screen_timer_width(resources: &ResourceBag) -> Val {
    if resources.screen_sizing.is_tablet() {
        Val::Pixel(150)
    } else {
        Val::Pixel(100)
    }
}
