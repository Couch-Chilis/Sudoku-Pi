use super::flex::*;
use crate::resource_bag::{ResourceBag, ResourceTuple};
use crate::utils::TransformExt;
use crate::{Screen, ScreenInteraction, ScreenState};
use bevy::{prelude::*, sprite::Anchor, window::WindowResized};
use smallvec::smallvec;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::DerefMut;

type FlexEntity<'a> = (
    Entity,
    &'a mut Transform,
    Option<&'a FlexItemStyle>,
    Option<&'a FlexContainerStyle>,
    Option<&'a mut ComputedPosition>,
    Option<&'a Children>,
    Option<&'a Screen>,
    Option<&'a mut Text>,
    Option<&'a FlexTextStyle>,
    Option<&'a Anchor>,
    Option<&'a ScreenInteraction>,
    Option<&'a DynamicImage>,
    Option<&'a mut Handle<Image>>,
);
type FlexQuery<'w, 's, 'a> = Query<'w, 's, FlexEntity<'a>, With<Flex>>;

type ItemMapEntry<'a> = (
    Cow<'a, FlexItemStyle>,
    Mut<'a, ComputedPosition>,
    Mut<'a, Transform>,
    Option<&'a ScreenInteraction>,
);

type TextMapEntry<'a> = (
    &'a Anchor,
    Mut<'a, Text>,
    &'a FlexTextStyle,
    Mut<'a, Transform>,
    Mut<'a, ComputedPosition>,
);

pub(crate) fn layout_system(
    mut flex_query: FlexQuery,
    changed_containers: Query<Entity, Changed<FlexContainerStyle>>,
    changed_items: Query<Entity, Changed<FlexItemStyle>>,
    resource_tuple: ResourceTuple,
    events: EventReader<WindowResized>,
) {
    if !changed_containers.is_empty() || !changed_items.is_empty() || !events.is_empty() {
        layout(&mut flex_query, &resource_tuple);
    }
}

fn layout(flex_query: &mut FlexQuery, resource_tuple: &ResourceTuple) {
    let mut layout_info =
        LayoutInfo::from_query(flex_query, ResourceBag::from_tuple(resource_tuple));
    layout_info.apply();
}

struct LayoutInfo<'a> {
    screens: Vec<(Entity, ScreenState, f32, f32)>,
    container_map: BTreeMap<Entity, (&'a Children, Cow<'a, FlexContainerStyle>)>,
    item_map: BTreeMap<Entity, ItemMapEntry<'a>>,
    text_map: BTreeMap<Entity, TextMapEntry<'a>>,
    resources: ResourceBag<'a>,
}

impl<'a> LayoutInfo<'a> {
    fn from_query(flex_query: &'a mut FlexQuery, resources: ResourceBag<'a>) -> Self {
        let mut screens = Vec::new();
        let mut container_map = BTreeMap::new();
        let mut item_map = BTreeMap::new();
        let mut text_map = BTreeMap::new();

        for (
            entity,
            transform,
            item_style,
            container_style,
            computed_position,
            children,
            screen,
            text,
            text_style,
            anchor,
            screen_interaction,
            dynamic_image,
            image_handle,
        ) in flex_query.iter_mut()
        {
            if let (Some(container_style), Some(children)) = (container_style, children) {
                let container_style = if !container_style.dynamic_styles.is_empty() {
                    let mut style = container_style.clone();
                    for enhance in &container_style.dynamic_styles {
                        enhance(&mut style, &resources);
                    }
                    Cow::Owned(style)
                } else {
                    Cow::Borrowed(container_style)
                };

                container_map.insert(entity, (children, container_style));

                if let Some(screen) = screen {
                    // Assumption: Screens act as our root containers and always get
                    //             aligned with the real screen/window viewport.
                    screens.push((entity, screen.state, screen.width, screen.height));
                }
            }

            match (text, text_style, anchor, item_style, computed_position) {
                (Some(text), Some(text_style), Some(anchor), _, Some(computed_position)) => {
                    text_map.insert(
                        entity,
                        (anchor, text, text_style, transform, computed_position),
                    );
                }
                (_, _, _, Some(item_style), Some(computed_position)) => {
                    let mut item_style = if !item_style.dynamic_styles.is_empty() {
                        let mut style = item_style.clone();
                        for enhance in &item_style.dynamic_styles {
                            enhance(&mut style, &resources);
                        }
                        Cow::Owned(style)
                    } else {
                        Cow::Borrowed(item_style)
                    };

                    if let (Some(dynamic_image), Some(mut image_handle)) =
                        (dynamic_image, image_handle)
                    {
                        let image = dynamic_image.get_image(&resources);
                        if *image_handle != image.handle {
                            *image_handle = image.handle.clone();
                        }

                        item_style = Cow::Owned(FlexItemStyle {
                            transform: Transform::from_2d_scale(
                                1. / image.width,
                                1. / image.height,
                            ),
                            ..(*item_style).clone()
                        });
                    }

                    item_map.insert(
                        entity,
                        (item_style, computed_position, transform, screen_interaction),
                    );
                }
                _ => {}
            }
        }

        Self {
            screens,
            container_map,
            item_map,
            text_map,
            resources,
        }
    }

    fn apply(&mut self) {
        for (entity, screen_state, width, height) in self.screens.clone() {
            let position = ComputedPosition {
                width,
                height,
                screens: smallvec![screen_state],
                x: 0.,
                y: 0.,
            };
            self.apply_container(entity, position, (width, height));
        }
    }

    fn apply_container(
        &mut self,
        entity: Entity,
        position: ComputedPosition,
        screen_size: (f32, f32),
    ) {
        let Some((children, container_style)) = self.container_map.remove(&entity) else {
            return;
        };

        let axes_scales = position.axis_scales(screen_size.0, screen_size.1);

        let direction = container_style.direction;
        let scaling = axes_scales.scaling_for_direction(direction);

        let cross = direction.cross();
        let cross_scaling = axes_scales.scaling_for_direction(cross);

        let padding_before_for_direction = container_style.padding.before_for_direction(direction);
        let padding_before = padding_before_for_direction.evaluate(&scaling);
        let padding_after_for_direction = container_style.padding.after_for_direction(direction);
        let padding_after = padding_after_for_direction.evaluate(&scaling);
        let padding = padding_before + padding_after;
        let cross_padding = container_style
            .padding
            .before_for_direction(cross)
            .evaluate(&cross_scaling)
            + container_style
                .padding
                .after_for_direction(cross)
                .evaluate(&cross_scaling);

        let num_gaps = match container_style.gap {
            Val::None => 0.,
            _ => {
                (children
                    .iter()
                    .filter_map(|entity| self.item_map.get(entity))
                    .filter(|(item_style, ..)| item_style.occupies_space)
                    .count()
                    .max(1)
                    - 1) as f32
            }
        };
        let initial_size = padding + num_gaps * container_style.gap.evaluate(&scaling);
        let base_grow = match container_style.gap {
            Val::Auto => num_gaps,
            _ => 0.,
        } + match padding_before_for_direction {
            Val::Auto => 1.,
            _ => 0.,
        } + match padding_after_for_direction {
            Val::Auto => 1.,
            _ => 0.,
        };
        let (total_size, total_grow, total_shrink) = children
            .iter()
            .filter_map(|entity| self.item_map.get(entity))
            .filter(|(item_style, ..)| item_style.occupies_space)
            .map(|(item_style, ..)| {
                (
                    item_style.flex_base.for_direction(direction),
                    item_style.margin.for_direction(direction),
                    item_style.flex_grow,
                    item_style.flex_shrink,
                )
            })
            .fold(
                (initial_size, base_grow, 0.),
                |(size, grow, shrink), (item_size, item_margin, item_grow, item_shrink)| {
                    (
                        size + item_size.evaluate(&scaling) + 2. * item_margin.evaluate(&scaling),
                        grow + item_grow,
                        shrink + item_shrink,
                    )
                },
            );

        // We keep track of the offset for positioning children along the axis.
        let mut offset = padding_before;

        let spare_size = 1. - total_size;
        if spare_size > 0. && padding_before_for_direction == Val::Auto {
            offset += spare_size / total_grow;
        }

        for item_entity in children {
            if let Some(text_entry) = self.text_map.remove(item_entity) {
                self.position_text(text_entry, &position);
                continue;
            }

            let Some((item_style, mut computed_position, mut transform, screen_interaction)) =
                self.item_map.remove(item_entity)
            else {
                continue;
            };

            // Start by assuming the base size.
            let flex_base = item_style.flex_base.clone();
            let mut scale = Vec3::new(
                flex_base.width.evaluate(&axes_scales.horizontal),
                flex_base.height.evaluate(&axes_scales.vertical),
                1.,
            );

            // Determine the margins along the main and cross axes.
            let margin = item_style
                .margin
                .for_direction(direction)
                .evaluate(&scaling);
            let cross_margin = item_style
                .margin
                .for_direction(cross)
                .evaluate(&cross_scaling);

            // Grow or shrink as needed and if allowed.
            if spare_size > 0. {
                if item_style.flex_grow > 0. {
                    let base_size = flex_base.for_direction(direction).evaluate(&scaling);
                    let mut item_size =
                        base_size + spare_size * item_style.flex_grow / total_grow.max(1.);

                    // Preserve the aspect ratio, if requested.
                    if item_style.preserve_aspect_ratio {
                        let base_cross_size =
                            flex_base.for_direction(cross).evaluate(&cross_scaling);
                        let mut cross_size = (item_size / base_size) * base_cross_size;

                        // Make sure we don't grow too large along the cross axis.
                        let total_cross_size = cross_size + 2. * cross_margin;
                        if total_cross_size > 1. {
                            let previous_cross_size = cross_size;
                            cross_size = 1. - 2. * cross_margin;
                            item_size *= cross_size / previous_cross_size;
                        }

                        if direction == FlexDirection::Column {
                            scale.x = cross_size;
                            scale.y = item_size;
                        } else {
                            scale.x = item_size;
                            scale.y = cross_size;
                        }
                    } else if direction == FlexDirection::Column {
                        scale.y = item_size;
                    } else {
                        scale.x = item_size;
                    }
                }
            } else if item_style.flex_shrink > 0. {
                let excess_size = total_size - 1.;
                let base_size = flex_base.for_direction(direction).evaluate(&scaling);
                let min_size = item_style
                    .min_size
                    .for_direction(direction)
                    .evaluate(&scaling);
                let mut item_size = (base_size
                    - excess_size * item_style.flex_shrink / total_shrink.max(1.))
                .max(min_size);

                // Preserve the aspect ratio, if requested.
                if item_style.preserve_aspect_ratio {
                    let base_cross_size = flex_base.for_direction(cross).evaluate(&cross_scaling);
                    let mut cross_size = (item_size / base_size) * base_cross_size;

                    // Make sure we don't shrink too small.
                    let min_cross_size = item_style
                        .min_size
                        .for_direction(cross)
                        .evaluate(&cross_scaling);
                    if cross_size < min_cross_size {
                        let previous_cross_size = cross_size;
                        cross_size = min_cross_size;
                        item_size *= cross_size / previous_cross_size;
                    }

                    if direction == FlexDirection::Column {
                        scale.x = cross_size;
                        scale.y = item_size;
                    } else {
                        scale.x = item_size;
                        scale.y = cross_size;
                    }
                } else if direction == FlexDirection::Column {
                    scale.y = item_size;
                } else {
                    scale.x = item_size;
                }
            }

            // An item that wants to grow and doesn't care about aspect ratio
            // may take all available space on the cross axis.
            if item_style.flex_grow > 0. && !item_style.preserve_aspect_ratio {
                if direction == FlexDirection::Column {
                    scale.x = 1.
                        - 2. * item_style.margin.width.evaluate(&cross_scaling)
                        - container_style.padding.left.evaluate(&cross_scaling)
                        - container_style.padding.right.evaluate(&cross_scaling);
                } else {
                    scale.y = 1.
                        - 2. * item_style.margin.height.evaluate(&cross_scaling)
                        - container_style.padding.top.evaluate(&cross_scaling)
                        - container_style.padding.bottom.evaluate(&cross_scaling);
                }
            }

            // Determine translation.
            let translation = if direction == FlexDirection::Column {
                let x = match item_style.align_self {
                    Alignment::Centered => 0.,
                    Alignment::End => 0.5 - cross_padding - cross_margin - 0.5 * scale.x,
                    Alignment::Start => -0.5 + cross_padding + cross_margin + 0.5 * scale.x,
                };
                let y = 0.5 - offset - margin - 0.5 * scale.y;
                Vec3::new(x, y, 1.)
            } else {
                let x = -0.5 + offset + margin + 0.5 * scale.x;
                let y = match item_style.align_self {
                    Alignment::Centered => 0.,
                    Alignment::End => -0.5 + cross_padding + cross_margin + 0.5 * scale.y,
                    Alignment::Start => 0.5 - cross_padding - cross_margin - 0.5 * scale.y,
                };
                Vec3::new(x, y, 1.)
            };

            let mut layout_transform = Transform {
                scale,
                translation,
                ..default()
            };

            // Apply custom transformation, if requested.
            if item_style.transform != Transform::IDENTITY {
                layout_transform.scale *= item_style.transform.scale;
                layout_transform.translation += item_style.transform.translation;
                layout_transform.rotation = item_style.transform.rotation;
            }

            // Set the child's transform.
            if *transform != layout_transform {
                *transform = layout_transform;
            }

            // Set the position for use by other containers, and store it in the
            // `ComputedPosition` for use by the interaction system.
            let item_position = position.transformed_with_screen_interaction(
                scale,
                translation,
                screen_interaction,
            );
            *computed_position = item_position.clone();

            // Update offset for the next child.
            if item_style.occupies_space {
                offset += 2. * margin
                    + match container_style.gap {
                        Val::Auto if spare_size > 0. => spare_size / total_grow,
                        _ => container_style.gap.evaluate(&scaling),
                    }
                    + match direction {
                        FlexDirection::Column => scale.y,
                        FlexDirection::Row => scale.x,
                    };
            }

            // Apply recursively in case the child item is also a container.
            self.apply_container(*item_entity, item_position, screen_size);
        }
    }

    fn position_text(
        &self,
        (anchor, mut text, text_style, mut transform, mut computed_position): TextMapEntry,
        position: &ComputedPosition,
    ) {
        let ComputedPosition { width, height, .. } = position;

        transform.scale = Vec3::new(0.5 / width, 0.5 / height, 1.);
        transform.translation = Vec3::new(
            match anchor {
                Anchor::CenterLeft | Anchor::BottomLeft | Anchor::TopLeft => -0.5,
                Anchor::CenterRight | Anchor::BottomRight | Anchor::TopRight => 0.5,
                _ => 0.,
            },
            0.,
            1.,
        );

        *computed_position = position.transformed(transform.scale, transform.translation);
        if !text_style.dynamic_styles.is_empty() {
            for enhance in &text_style.dynamic_styles {
                for section in &mut text.sections {
                    enhance(&mut section.style, &self.resources);
                }
            }
        } else {
            text.deref_mut(); // Prevent text appearing too small on iOS.
        }
    }
}
