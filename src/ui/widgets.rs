use std::sync::Arc;

use bevy::prelude::*;

use crate::{assets::*, themes::Theme, utils::*, ResourceBag};

use super::{
    buttons::*, child_spawner_commands_ext::*, flex::*, interaction::*, props::*,
    style_enhancers::*,
};

pub fn primary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
{
    |props, spawner| {
        let mut style = FlexItemStyle::default();
        styles.enhance(&mut style);

        let bundle = ButtonBundle::from_theme_and_style(&props.resources.theme, style);

        spawner.spawn((action, bundle)).with_children(|spawner| {
            spawner.spawn_with_children(props, child);
        });
    }
}

pub fn selected_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> (impl Bundle, impl FnOnce(&Props, &mut ChildSpawnerCommands))
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

    let spawn_children = |props: &Props, spawner: &mut ChildSpawnerCommands| {
        spawner.spawn_with_children(props, child);
    };

    ((action, InitialSelection, bundle), spawn_children)
}

pub fn secondary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
{
    |props, spawner| {
        let mut style = FlexItemStyle::default();
        styles.enhance(&mut style);

        let bundle = ButtonBundle {
            flex: FlexBundle::new(
                style,
                FlexContainerStyle::row().with_padding(Sides::all(BORDER_THICKNESS)),
            )
            .with_background_color(props.resources.theme.secondary_button_border),
            ..default()
        };

        spawner
            .spawn((action, ButtonType::Secondary, bundle))
            .with_children(|spawner| {
                spawner
                    .spawn((
                        ButtonBackground,
                        FlexBundle::new(
                            FlexItemStyle::available_size().without_occupying_space(),
                            FlexContainerStyle::row(),
                        )
                        .with_background_color(props.resources.theme.secondary_button_background),
                    ))
                    .with_children(|spawner| {
                        spawner.spawn_with_children(props, child);
                    });
            });
    }
}

pub fn ternary_button<B>(
    action: impl Bundle,
    styles: impl FlexItemStyleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
{
    |props, spawner| {
        let mut style = FlexItemStyle::default();
        styles.enhance(&mut style);

        let bundle = ButtonBundle {
            flex: FlexBundle::new(
                style,
                FlexContainerStyle::row().with_padding(Sides::all(BORDER_THICKNESS)),
            )
            .with_background_color(props.resources.theme.ternary_button_border),
            ..default()
        };

        spawner
            .spawn((action, ButtonType::Ternary, bundle))
            .with_children(|spawner| {
                spawner
                    .spawn((
                        ButtonBackground,
                        FlexBundle::new(
                            FlexItemStyle::available_size().without_occupying_space(),
                            FlexContainerStyle::row(),
                        )
                        .with_background_color(props.resources.theme.ternary_button_background),
                    ))
                    .with_children(|spawner| {
                        spawner.spawn_with_children(props, child);
                    });
            });
    }
}

pub fn column<B>(
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
{
    column_t((), item_styles, container_styles, child)
}

pub fn column_t<B, T>(
    marker: T,
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
    T: Bundle,
{
    |props, spawner| {
        let mut bundle = FlexBundle::default();
        item_styles.enhance(&mut bundle.item.style);
        container_styles.enhance(&mut bundle.container, &props.resources);

        spawner.spawn((bundle, marker)).with_children(|spawner| {
            spawner.spawn_with_children(props, child);
        });
    }
}

pub fn dynamic_image<T>(dynamic_image: T, styles: impl FlexItemStyleEnhancer) -> impl Bundle
where
    T: Fn(Mut<Sprite>, &ResourceBag) -> (f32, f32) + Send + Sync + 'static,
{
    let mut item = FlexItemBundle::default();
    styles.enhance(&mut item.style);

    let flex_image = FlexImageBundle {
        dynamic_image: DynamicImage(Arc::new(dynamic_image)),
    };

    ((item, flex_image), Sprite::default())
}

pub fn fragment<B1, B2>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B1: Bundle,
    B2: Bundle,
{
    |props, spawner| {
        spawner.spawn_with_children(props, child1);
        spawner.spawn_with_children(props, child2);
    }
}

pub fn fragment3<B1, B2, B3>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
{
    |props, spawner| {
        spawner.spawn_with_children(props, child1);
        spawner.spawn_with_children(props, child2);
        spawner.spawn_with_children(props, child3);
    }
}

pub fn fragment4<B1, B2, B3, B4>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
{
    |props, spawner| {
        spawner.spawn_with_children(props, child1);
        spawner.spawn_with_children(props, child2);
        spawner.spawn_with_children(props, child3);
        spawner.spawn_with_children(props, child4);
    }
}

pub fn fragment5<B1, B2, B3, B4, B5>(
    child1: impl Into<BundleWithChildren<B1>>,
    child2: impl Into<BundleWithChildren<B2>>,
    child3: impl Into<BundleWithChildren<B3>>,
    child4: impl Into<BundleWithChildren<B4>>,
    child5: impl Into<BundleWithChildren<B5>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
{
    |props, spawner| {
        spawner.spawn_with_children(props, child1);
        spawner.spawn_with_children(props, child2);
        spawner.spawn_with_children(props, child3);
        spawner.spawn_with_children(props, child4);
        spawner.spawn_with_children(props, child5);
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
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
    B6: Bundle,
    B7: Bundle,
{
    |props, spawner| {
        spawner.spawn_with_children(props, child1);
        spawner.spawn_with_children(props, child2);
        spawner.spawn_with_children(props, child3);
        spawner.spawn_with_children(props, child4);
        spawner.spawn_with_children(props, child5);
        spawner.spawn_with_children(props, child6);
        spawner.spawn_with_children(props, child7);
    }
}

pub fn image(image: ImageWithDimensions, styles: impl FlexItemStyleEnhancer) -> impl Bundle {
    let mut item = FlexItemBundle::default();
    styles.enhance(&mut item.style);
    item.style.transform = Transform::from_2d_scale(1. / image.width, 1. / image.height);

    let sprite = Sprite::from_image(image.handle);

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
    bundle.sprite = Sprite::from_color(Color::NONE, Vec2::new(1., 1.));
    styles.enhance(&mut bundle.flex.style);
    bundle
}

pub fn row<B>(
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
{
    row_t((), item_styles, container_styles, child)
}

pub fn row_t<B, T>(
    marker: T,
    item_styles: impl FlexItemStyleEnhancer,
    container_styles: impl FlexContainerBundleEnhancer,
    child: impl Into<BundleWithChildren<B>>,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands)
where
    B: Bundle,
    T: Bundle,
{
    |props, spawner| {
        let mut bundle = FlexBundle::default();
        bundle.container.style.direction = FlexDirection::Row;
        item_styles.enhance(&mut bundle.item.style);
        container_styles.enhance(&mut bundle.container, &props.resources);

        spawner.spawn((bundle, marker)).with_children(|spawner| {
            spawner.spawn_with_children(props, child);
        });
    }
}

pub fn text(
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    |props, spawner| {
        let mut bundle = FlexTextBundle::from_text(text);
        styles.enhance(&mut bundle, &props.resources);
        spawner.spawn(bundle);
    }
}

pub fn text_t(
    marker: impl Bundle,
    text: impl Into<String>,
    styles: impl TextEnhancer,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    |props, spawner| {
        let mut bundle = FlexTextBundle::from_text(text);
        styles.enhance(&mut bundle, &props.resources);
        spawner.spawn((bundle, marker));
    }
}

pub fn theme_rect(
    selector: impl FnOnce(&Theme) -> Color,
    styles: impl FlexItemStyleEnhancer,
) -> impl FnOnce(&Props, &mut ChildSpawnerCommands) {
    |props, spawner| {
        let mut item = FlexItemBundle::default();
        styles.enhance(&mut item.style);

        let sprite = Sprite::from_color(selector(props.resources.theme), Vec2::new(1., 1.));

        spawner.spawn((item, sprite));
    }
}
