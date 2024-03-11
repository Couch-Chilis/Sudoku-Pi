use super::{
    buttons::*, child_builder_ext::*, flex::*, interaction::*, props::*, style_enhancers::*,
};
use crate::ResourceBag;
use crate::{assets::*, constants::*, utils::*};
use bevy::prelude::*;
use std::sync::Arc;

pub fn primary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle::from_style(style);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    ((action, bundle), spawn_children)
}

pub fn selected_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle {
        interaction: Interaction::Selected,
        flex: FlexBundle::from_item_style(style),
        ..default()
    };

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    ((action, InitialSelection, bundle), spawn_children)
}

pub fn secondary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle {
        flex: FlexBundle::new(
            style,
            FlexContainerStyle::row().with_padding(Sides::all(BORDER_THICKNESS)),
        )
        .with_background_color(COLOR_SECONDARY_BUTTON_BORDER),
        ..default()
    };

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn((
            ButtonBackground,
            FlexBundle::new(
                FlexItemStyle::available_size().without_occupying_space(),
                FlexContainerStyle::row(),
            )
            .with_background_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
        ))
        .with_children(|child_builder| {
            child_builder.spawn_with_children(props, child);
        });
    };

    ((action, ButtonType::Secondary, bundle), spawn_children)
}

pub fn ternary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle {
        flex: FlexBundle::new(
            style,
            FlexContainerStyle::row().with_padding(Sides::all(BORDER_THICKNESS)),
        )
        .with_background_color(COLOR_TERNARY_BUTTON_BORDER),
        ..default()
    };

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn((
            ButtonBackground,
            FlexBundle::new(
                FlexItemStyle::available_size().without_occupying_space(),
                FlexContainerStyle::row(),
            )
            .with_background_color(COLOR_TERNARY_BUTTON_BACKGROUND),
        ))
        .with_children(|child_builder| {
            child_builder.spawn_with_children(props, child);
        });
    };

    ((action, ButtonType::Ternary, bundle), spawn_children)
}

pub fn column<B>(
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexBundle::default();
    item_styles.enhance(&mut bundle.item.style);
    container_styles.enhance(&mut bundle.container);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
}

pub fn column_t<B, T>(
    marker: T,
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
    T: Bundle,
{
    let (bundle, spawn_children) = column(item_styles, container_styles, child);

    ((bundle, marker), spawn_children)
}

pub fn container<B>(
    styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (FlexContainerBundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexContainerBundle::default();
    styles.enhance(&mut bundle);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
}

pub fn dynamic_image<T>(dynamic_image: T, styles: impl FlexItemStyleEnhancer) -> impl Bundle
where
    T: Fn(Mut<'_, Handle<Image>>, &ResourceBag) -> (f32, f32) + Send + Sync + 'static,
{
    let mut item = FlexItemBundle::default();
    styles.enhance(&mut item.style);

    let flex_image = FlexImageBundle {
        dynamic_image: DynamicImage(Arc::new(dynamic_image)),
    };

    ((item, flex_image), SpriteBundle::default())
}

pub fn fragment<B1, B2>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
) -> impl FnOnce(&Props, &mut ChildBuilder)
where
    B1: Bundle,
    B2: Bundle,
{
    |props, cb| {
        cb.spawn_with_children(props, child1);
        cb.spawn_with_children(props, child2);
    }
}

pub fn fragment3<B1, B2, B3>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
) -> impl FnOnce(&Props, &mut ChildBuilder)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
{
    |props, cb| {
        cb.spawn_with_children(props, child1);
        cb.spawn_with_children(props, child2);
        cb.spawn_with_children(props, child3);
    }
}

pub fn fragment4<B1, B2, B3, B4>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
) -> impl FnOnce(&Props, &mut ChildBuilder)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
{
    |props, cb| {
        cb.spawn_with_children(props, child1);
        cb.spawn_with_children(props, child2);
        cb.spawn_with_children(props, child3);
        cb.spawn_with_children(props, child4);
    }
}

pub fn fragment5<B1, B2, B3, B4, B5>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
    child5: impl Into<BundleWithChildren<B5>>,
) -> impl FnOnce(&Props, &mut ChildBuilder)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
{
    |props, cb| {
        cb.spawn_with_children(props, child1);
        cb.spawn_with_children(props, child2);
        cb.spawn_with_children(props, child3);
        cb.spawn_with_children(props, child4);
        cb.spawn_with_children(props, child5);
    }
}

pub fn fragment7<B1, B2, B3, B4, B5, B6, B7>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
    child5: impl Into<BundleWithChildren<B5>>,
    child6: impl Into<BundleWithChildren<B6>>,
    child7: impl Into<BundleWithChildren<B7>>,
) -> impl FnOnce(&Props, &mut ChildBuilder)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
    B6: Bundle,
    B7: Bundle,
{
    |props, cb| {
        cb.spawn_with_children(props, child1);
        cb.spawn_with_children(props, child2);
        cb.spawn_with_children(props, child3);
        cb.spawn_with_children(props, child4);
        cb.spawn_with_children(props, child5);
        cb.spawn_with_children(props, child6);
        cb.spawn_with_children(props, child7);
    }
}

pub fn image(image: ImageWithDimensions, styles: impl FlexItemStyleEnhancer) -> impl Bundle {
    let mut item = FlexItemBundle::default();
    styles.enhance(&mut item.style);
    item.style.transform = Transform::from_2d_scale(1. / image.width, 1. / image.height);

    let sprite = SpriteBundle {
        texture: image.handle,
        ..default()
    };

    (item, sprite)
}

pub fn image_t<T>(
    marker: T,
    image_with_dimensions: ImageWithDimensions,
    styles: impl FlexItemStyleEnhancer,
) -> impl Bundle
where
    T: Bundle,
{
    (image(image_with_dimensions, styles), marker)
}

pub fn leaf(styles: impl FlexItemStyleEnhancer) -> FlexLeafBundle {
    let mut bundle = FlexLeafBundle::default();
    styles.enhance(&mut bundle.flex.style);
    bundle
}

pub fn rect(color: Color, styles: impl FlexItemStyleEnhancer) -> impl Bundle {
    let mut item = FlexItemBundle::default();
    styles.enhance(&mut item.style);

    let sprite = SpriteBundle {
        sprite: Sprite::from_color(color),
        ..default()
    };

    (item, sprite)
}

pub fn row<B>(
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexBundle::default();
    bundle.container.style.direction = FlexDirection::Row;
    item_styles.enhance(&mut bundle.item.style);
    container_styles.enhance(&mut bundle.container);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
}

pub fn row_t<B, T>(
    marker: T,
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
    T: Bundle,
{
    let (bundle, spawn_children) = row(item_styles, container_styles, child);

    ((bundle, marker), spawn_children)
}

pub fn text(
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    |props: &Props, cb: &mut ChildBuilder| {
        let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
        styles.enhance(&mut bundle.text, &props.resources);
        cb.spawn(bundle);
    }
}

pub fn text_t(
    marker: impl Bundle,
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    |props: &Props, cb: &mut ChildBuilder| {
        let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
        styles.enhance(&mut bundle.text, &props.resources);
        cb.spawn((bundle, marker));
    }
}
