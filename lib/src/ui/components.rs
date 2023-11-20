use super::{buttons::*, flex::*, interaction::*, FlexItemStyleEnhancer};
use crate::constants::*;
use bevy::prelude::*;

pub fn primary_button(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Bundle,
) -> (impl Bundle, impl FnOnce(&mut ChildBuilder)) {
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle::from_style(style);

    let spawn_children = |child_builder: &mut ChildBuilder| {
        child_builder.spawn(child);
    };

    ((action, bundle), spawn_children)
}

pub fn selected_button(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Bundle,
) -> (impl Bundle, impl FnOnce(&mut ChildBuilder)) {
    let mut style = FlexItemStyle::default();
    styles.enhance(&mut style);

    let bundle = ButtonBundle {
        interaction: Interaction::Selected,
        flex: FlexBundle::from_item_style(style),
        ..default()
    };

    let spawn_children = |child_builder: &mut ChildBuilder| {
        child_builder.spawn(child);
    };

    ((action, InitialSelection, bundle), spawn_children)
}

pub fn secondary_button(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Bundle,
) -> (impl Bundle, impl FnOnce(&mut ChildBuilder)) {
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

    let spawn_children = |child_builder: &mut ChildBuilder| {
        child_builder
            .spawn((
                ButtonBackground,
                FlexBundle::new(
                    FlexItemStyle::available_size().without_occupying_space(),
                    FlexContainerStyle::row(),
                )
                .with_background_color(COLOR_SECONDARY_BUTTON_BACKGROUND),
            ))
            .with_children(|child_builder| {
                child_builder.spawn(child);
            });
    };

    ((action, ButtonType::Secondary, bundle), spawn_children)
}

pub fn ternary_button(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Bundle,
) -> (impl Bundle, impl FnOnce(&mut ChildBuilder)) {
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

    let spawn_children = |child_builder: &mut ChildBuilder| {
        child_builder
            .spawn((
                ButtonBackground,
                FlexBundle::new(
                    FlexItemStyle::available_size().without_occupying_space(),
                    FlexContainerStyle::row(),
                )
                .with_background_color(COLOR_TERNARY_BUTTON_BACKGROUND),
            ))
            .with_children(|child_builder| {
                child_builder.spawn(child);
            });
    };

    ((action, ButtonType::Ternary, bundle), spawn_children)
}

pub fn leaf(attrs: impl Fn(&mut FlexItemStyle)) -> impl Bundle {
    let mut bundle = FlexLeafBundle::default();
    attrs(&mut bundle.flex.style);
    bundle
}

pub fn text(text: impl Into<String>, attrs: impl FnOnce(&mut TextStyle)) -> impl Bundle {
    let mut bundle = FlexTextBundle::from_text(Text::from_section(text, TextStyle::default()));
    attrs(&mut bundle.text.text.sections[0].style);
    bundle
}
