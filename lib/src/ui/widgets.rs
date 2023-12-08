use super::{
    buttons::*, child_builder_ext::*, flex::*, interaction::*, props::*, style_enhancers::*,
};
use crate::constants::*;
use bevy::prelude::*;

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
    container_styles: impl FlexContainerStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexBundle::default();
    item_styles.enhance(&mut bundle.item.style);
    container_styles.enhance(&mut bundle.container.style);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
}

pub fn container<B>(
    styles: impl FlexContainerStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (FlexContainerBundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexContainerBundle::default();
    styles.enhance(&mut bundle.style);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
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

pub fn leaf(styles: impl FlexItemStyleEnhancer) -> FlexLeafBundle {
    let mut bundle = FlexLeafBundle::default();
    styles.enhance(&mut bundle.flex.style);
    bundle
}

pub fn row<B>(
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildBuilder))
where
    B: Bundle,
{
    let mut bundle = FlexBundle::default();
    bundle.container.style.direction = FlexDirection::Row;
    item_styles.enhance(&mut bundle.item.style);
    container_styles.enhance(&mut bundle.container.style);

    let spawn_children = |props: &Props, cb: &mut ChildBuilder| {
        cb.spawn_with_children(props, child);
    };

    (bundle, spawn_children)
}

pub fn text(
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    text_with_bundle_enhancer(text, styles, |_bundle| {})
}

pub fn text_with_bundle_enhancer(
    text: impl Into<String>,
    styles: impl TextEnhancer,
    enhance: impl FnOnce(&mut Text2dBundle),
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    |props: &Props, cb: &mut ChildBuilder| {
        let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
        enhance(&mut bundle.text);
        styles.enhance(&mut bundle.text.text, &props.resources);
        cb.spawn(bundle);
    }
}

pub fn text_with_marker(
    marker: impl Bundle,
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    text_with_marker_and_bundle_enhancer(marker, text, styles, |_bundle| {})
}

pub fn text_with_marker_and_bundle_enhancer(
    marker: impl Bundle,
    text: impl Into<String>,
    styles: impl TextEnhancer,
    enhance: impl FnOnce(&mut Text2dBundle),
) -> impl FnOnce(&Props, &mut ChildBuilder) {
    |props: &Props, cb: &mut ChildBuilder| {
        let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
        enhance(&mut bundle.text);
        styles.enhance(&mut bundle.text.text, &props.resources);
        cb.spawn((bundle, marker));
    }
}
