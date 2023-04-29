use super::flex_bundles::*;
use crate::Screen;
use bevy::{prelude::*, window::WindowResized};
use std::collections::BTreeMap;

type FlexEntity<'a> = (
    Entity,
    &'a mut Transform,
    Option<&'a FlexItemStyle>,
    Option<&'a FlexContainerStyle>,
    Option<&'a mut ComputedPosition>,
    Option<&'a Parent>,
    Option<&'a Screen>,
);
type FlexQuery<'w, 's, 'a> = Query<'w, 's, FlexEntity<'a>, With<Flex>>;

/// Note we do not support dynamic changing of containers or items. This is not
/// a problem for us, since all the UI layouts are created at
pub(crate) fn ui_layout_system(
    changed_containers: Query<Entity, (With<FlexContainerStyle>, Changed<Children>)>,
    mut flex_query: FlexQuery,
) {
    if !changed_containers.is_empty() {
        layout(&mut flex_query);
    }
}

pub(crate) fn on_resize_layout(events: EventReader<WindowResized>, mut flex_query: FlexQuery) {
    if !events.is_empty() {
        layout(&mut flex_query);
    }
}

fn layout(flex_query: &mut FlexQuery) {
    let mut containers: Vec<(Entity, FlexContainerStyle)> = Vec::new();
    let mut children_map: BTreeMap<Entity, Vec<Entity>> = BTreeMap::new();
    let mut item_map: BTreeMap<Entity, (&FlexItemStyle, Mut<ComputedPosition>, Mut<Transform>)> =
        BTreeMap::new();
    let mut position_map: BTreeMap<Entity, ComputedPosition> = BTreeMap::new();

    for (entity, transform, item_style, container_style, computed_position, parent, screen) in
        flex_query.iter_mut()
    {
        if let Some(container_style) = container_style {
            containers.push((entity, container_style.clone()));
        }

        if let Some(screen) = screen {
            // Assumption: Screens only get translated, so determining their
            // computed position is easy.
            let position = ComputedPosition {
                width: screen.width,
                height: screen.height,
                x: transform.translation.x,
                y: transform.translation.y,
            };
            position_map.insert(entity, position);
        }

        if let Some(parent) = parent {
            children_map
                .entry(parent.get())
                .and_modify(|children| children.push(entity))
                .or_insert(vec![entity]);
        }

        if let (Some(item_style), Some(computed_position)) = (item_style, computed_position) {
            item_map.insert(entity, (item_style, computed_position, transform));
        }
    }

    // Assumption: We expect entity IDs to correlate to the order in which their
    // entities appear in the tree. This way, a simple sort is enough to
    // guarantee a top-down iteration order.
    containers.sort_by_key(|(entity, _)| *entity);

    for (container_entity, container_style) in containers {
        let Some(mut children) = children_map.remove(&container_entity) else {
            bevy::log::warn!("No children for container {container_entity:?}");
            return;
        };
        children.sort();

        let get_main_axis = if container_style.direction == FlexDirection::Column {
            Size::height
        } else {
            Size::width
        };
        let get_cross_axis = if container_style.direction == FlexDirection::Column {
            Size::width
        } else {
            Size::height
        };
        let get_main_scale = if container_style.direction == FlexDirection::Column {
            |vec: Vec2| vec.y
        } else {
            |vec: Vec2| vec.x
        };
        let get_cross_scale = if container_style.direction == FlexDirection::Column {
            |vec: Vec2| vec.x
        } else {
            |vec: Vec2| vec.y
        };

        let container_position = position_map.remove(&container_entity);
        let vmin_scales = container_position
            .as_ref()
            .map(|value| value.vmin_scales())
            .unwrap_or(Vec2::new(0.01, 0.01));

        let vmin_scale = get_main_scale(vmin_scales);
        let total_size = 2. * get_main_axis(&container_style.padding).evaluate(vmin_scale)
            + children
                .iter()
                .filter_map(|entity| item_map.get(entity))
                .map(|(item_style, _, _)| {
                    (
                        get_main_axis(&item_style.flex_base),
                        get_main_axis(&item_style.margin),
                    )
                })
                .fold(0., |acc, (size, margin)| {
                    acc + size.evaluate(vmin_scale) + 2. * margin.evaluate(vmin_scale)
                });

        // We keep track of the offset for positioning children along the axis.
        let mut offset = 0.;

        // Shortcut: For now we assume only one item may want to grow or shrink,
        // so we can do this in a single pass.
        for item_entity in children {
            let Some((item_style, mut computed_position, mut transform)) =
                item_map.remove(&item_entity) else {
                    bevy::log::warn!("No entry found for child entity {item_entity:?}");
                    continue;
                };

            // Start by assuming the base size.
            let mut scale = Vec3::new(
                item_style.flex_base.width.evaluate(vmin_scales.x),
                item_style.flex_base.height.evaluate(vmin_scales.y),
                1.,
            );

            // Grow or shrink as needed and if allowed.
            if total_size < 1. {
                if item_style.flex_grow > 0. {
                    let spare_size = 1. - total_size;
                    let base_size = get_main_axis(&item_style.flex_base).evaluate(vmin_scale);
                    let mut item_size = base_size + spare_size;

                    // Preserve the aspect ratio, if requested.
                    if item_style.preserve_aspect_ratio {
                        let cross_scale = get_cross_scale(vmin_scales);
                        let base_cross_size =
                            get_cross_axis(&item_style.flex_base).evaluate(cross_scale);
                        let mut cross_size = (item_size / base_size) * base_cross_size;

                        // Make sure we don't grow too large along the cross axis.
                        let cross_margin = get_cross_axis(&item_style.margin).evaluate(cross_scale);
                        let total_cross_size = cross_size + 2. * cross_margin;
                        if total_cross_size > 1. {
                            let previous_cross_size = cross_size;
                            cross_size = 1. - 2. * cross_margin;
                            item_size *= cross_size / previous_cross_size;
                        }

                        if container_style.direction == FlexDirection::Column {
                            scale.x = cross_size;
                            scale.y = item_size;
                        } else {
                            scale.x = item_size;
                            scale.y = cross_size;
                        }
                    } else if container_style.direction == FlexDirection::Column {
                        scale.y = item_size;
                    } else {
                        scale.x = item_size;
                    }
                }
            } else if item_style.flex_shrink > 0. {
                let excess_size = total_size - 1.;
                let base_size = get_main_axis(&item_style.flex_base).evaluate(vmin_scale);
                let min_size = get_main_axis(&item_style.min_size).evaluate(vmin_scale);
                let mut item_size = (base_size - excess_size).max(min_size);

                // Preserve the aspect ratio, if requested.
                if item_style.preserve_aspect_ratio {
                    let cross_scale = get_cross_scale(vmin_scales);
                    let base_cross_size =
                        get_cross_axis(&item_style.flex_base).evaluate(cross_scale);
                    let mut cross_size = (item_size / base_size) * base_cross_size;

                    // Make sure we don't shrink too small.
                    let min_cross_size = get_cross_axis(&item_style.min_size).evaluate(cross_scale);
                    if cross_size < min_cross_size {
                        let previous_cross_size = cross_size;
                        cross_size = min_cross_size;
                        item_size *= cross_size / previous_cross_size;
                    }

                    if container_style.direction == FlexDirection::Column {
                        scale.x = cross_size;
                        scale.y = item_size;
                    } else {
                        scale.x = item_size;
                        scale.y = cross_size;
                    }
                } else if container_style.direction == FlexDirection::Column {
                    scale.y = item_size;
                } else {
                    scale.x = item_size;
                }
            }

            // An item that wants to grow and doesn't care about aspect ratio,
            // may take all available space on the cross axis.
            if item_style.flex_grow > 0. && !item_style.preserve_aspect_ratio {
                if container_style.direction == FlexDirection::Column {
                    scale.x = 1.;
                } else {
                    scale.y = 1.;
                }
            }

            // Determine the main axis margin.
            let margin = get_main_axis(&item_style.margin).evaluate(vmin_scale);

            // Determine translation.
            let translation = if container_style.direction == FlexDirection::Column {
                Vec3::new(0., 0.5 - offset - margin - 0.5 * scale.y, 1.)
            } else {
                Vec3::new(-0.5 + offset + margin + 0.5 * scale.x, 0., 1.)
            };

            // Set the child's transform.
            if transform.scale != scale || transform.translation != translation {
                transform.scale = scale;

                // Be careful not to lower any Z-indices, or sprites may not
                // be shown anymore.
                transform.translation = Vec3 {
                    z: transform.translation.z.max(translation.z),
                    ..translation
                };
            }

            // Set the position for use by other containers, and store it in the
            // `ComputedPosition` for use by the interaction system.
            if let Some(container_position) = container_position.as_ref() {
                let item_position = container_position.transformed(scale, translation);
                *computed_position = item_position;
                position_map.insert(item_entity, item_position);
            } else {
                bevy::log::warn!("Cannot set computed position on {item_entity:?}");
            }

            // Update offset for the next child.
            offset += 2. * margin
                + if container_style.direction == FlexDirection::Column {
                    scale.y
                } else {
                    scale.x
                };
        }
    }
}
