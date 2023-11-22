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

pub fn center_text(text: impl Into<String>, styles: impl TextStyleEnhancer) -> FlexTextBundle {
    let mut bundle = FlexTextBundle::from_text(
        Text::from_section(text, TextStyle::default()).with_alignment(TextAlignment::Center),
    );
    styles.enhance(&mut bundle.text.text.sections[0].style);
    bundle
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

pub fn fragment<B1, B2>(
    props: &Props,
    cb: &mut ChildBuilder,
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
) where
    B1: Bundle,
    B2: Bundle,
{
    cb.spawn_with_children(props, child1);
    cb.spawn_with_children(props, child2);
}

pub fn fragment3<B1, B2, B3>(
    props: &Props,
    cb: &mut ChildBuilder,
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
) where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
{
    cb.spawn_with_children(props, child1);
    cb.spawn_with_children(props, child2);
    cb.spawn_with_children(props, child3);
}

pub fn fragment4<B1, B2, B3, B4>(
    props: &Props,
    cb: &mut ChildBuilder,
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
) where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
{
    cb.spawn_with_children(props, child1);
    cb.spawn_with_children(props, child2);
    cb.spawn_with_children(props, child3);
    cb.spawn_with_children(props, child4);
}

pub fn fragment5<B1, B2, B3, B4, B5>(
    props: &Props,
    cb: &mut ChildBuilder,
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
    child5: impl Into<BundleWithChildren<B5>>,
) where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
{
    cb.spawn_with_children(props, child1);
    cb.spawn_with_children(props, child2);
    cb.spawn_with_children(props, child3);
    cb.spawn_with_children(props, child4);
    cb.spawn_with_children(props, child5);
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

pub fn text(text: impl Into<String>, styles: impl TextStyleEnhancer) -> FlexTextBundle {
    let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
    styles.enhance(&mut bundle.text.text.sections[0].style);
    bundle
}
