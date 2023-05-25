use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};
use std::ops::Mul;

use crate::utils::TransformExt;

/// Marker for any flex entity, be it an item or a container.
#[derive(Clone, Component, Default)]
pub struct Flex;

/// Convenience bundle for creating flex entities that are both containers and
/// items, for pure layout purposes.
#[derive(Bundle, Clone, Default)]
pub struct FlexBundle {
    pub container: FlexContainerBundle,
    pub item: FlexItemBundle,
}

impl FlexBundle {
    pub fn from_item_style(item_style: FlexItemStyle) -> Self {
        Self::new(FlexContainerStyle::default(), item_style)
    }

    pub fn new(container_style: FlexContainerStyle, item_style: FlexItemStyle) -> Self {
        Self {
            container: FlexContainerBundle {
                style: container_style,
                ..default()
            },
            item: FlexItemBundle {
                style: item_style,
                ..default()
            },
        }
    }
}

/// A layout bundle based on the flex system, though we're only bothering with
/// the parts relevant to us. Flex containers calculate the position of their
/// children automatically. Currently, container only work reliably if they're
/// added to entities with a `Screen` component or other flex items.
///
/// The main reason for using a custom system instead of Bevy UI is because we
/// need components that can be mixed and matched with non-UI entities, to allow
/// custom transforms on them.
#[derive(Bundle, Clone)]
pub struct FlexContainerBundle {
    pub style: FlexContainerStyle,
    pub background: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for FlexContainerBundle {
    fn default() -> Self {
        Self {
            style: FlexContainerStyle::default(),
            background: Default::default(),
            transform: Transform::default_2d(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

#[derive(Clone, Component, Default)]
pub struct FlexContainerStyle {
    /// Direction to lay out children.
    pub direction: FlexDirection,

    /// Gap to display between items.
    ///
    /// Using `Val::Auto` will distribute any remaining space evenly across the
    /// space between items. The effect would be the same as if between every
    /// item there was an empty "spacer" item with `flex_grow: 1.`.
    pub gap: Val,

    /// Padding to keep within the container and around the items.
    pub padding: Size,
}

impl FlexContainerStyle {
    pub fn row() -> Self {
        Self {
            direction: FlexDirection::Row,
            ..default()
        }
    }

    pub fn with_gap(self, gap: Val) -> Self {
        Self { gap, ..self }
    }

    pub fn with_padding(self, padding: Size) -> Self {
        Self { padding, ..self }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum FlexDirection {
    #[default]
    Column,
    Row,
}

impl FlexDirection {
    /// Returns the cross, or perpendicular, direction.
    pub fn cross(&self) -> Self {
        match self {
            Self::Column => Self::Row,
            Self::Row => Self::Column,
        }
    }
}

/// A layout bundle based on the flex system, though we're only bothering with
/// the parts relevant to us.
///
/// The main reason for using a custom system instead of Bevy UI is because we
/// need components that can be mixed and matched with non-UI entities, to allow
/// custom transforms on them.
///
/// Flex items should be attached to renderable entities, rather than inserting
/// the entities as children of flex items. Inserting children is allowed too,
/// but those entities will not have a `ComputedPosition` and thus won't be
/// usable with the `Interaction` component or be able to host further nested
/// flex containers.
///
/// A single entity may act as both flex item and container, which is the
/// recommended way of nesting layouts.
#[derive(Bundle, Clone)]
pub struct FlexItemBundle {
    pub flex: Flex,
    pub style: FlexItemStyle,
    pub computed_position: ComputedPosition,
}

impl FlexItemBundle {
    pub fn from_style(style: FlexItemStyle) -> Self {
        Self { style, ..default() }
    }
}

impl Default for FlexItemBundle {
    fn default() -> Self {
        Self {
            flex: Flex,
            style: FlexItemStyle::default(),
            computed_position: Default::default(),
        }
    }
}

#[derive(Clone, Component)]
pub struct FlexItemStyle {
    /// How the item should be aligned along the container's cross axis.
    pub align_self: Alignment,

    /// The base size that should be reserved for this item.
    pub flex_base: Size,

    /// The ratio with which this item may want to grow if there's space
    /// available.
    pub flex_grow: f32,

    /// The ratio with which this item may want to shrink if there's not enough
    /// space available.
    pub flex_shrink: f32,

    /// Margin to keep around the item.
    pub margin: Size,

    /// Minimum size to accept in case of shrinking.
    pub min_size: Size,

    /// Whether this item occupies space. If `false`, this item does not count
    /// towards the total space taken by the items inside a container and the
    /// next item (if any) may be rendered in exactly the same space. This can
    /// still be useful if you use a transform to move the item elsewhere, for
    /// instance.
    pub occupies_space: bool,

    /// Set to `true` if aspect ratio relative to the base size must be
    /// preserved in case of shrinking or growing.
    pub preserve_aspect_ratio: bool,

    /// A custom transform to apply on top of the transform determined by the
    /// layout system. Do note that the computed position of the item does *not*
    /// take this transform into account, so if interaction is required inside
    /// this item, there may be a mismatch in coordinates.
    pub transform: Transform,
}

impl FlexItemStyle {
    /// Returns the style for an item that has no base size, but takes all the
    /// space that is available.
    pub fn available_size() -> Self {
        Self {
            flex_grow: 1.,
            ..default()
        }
    }

    /// Returns the style for an item with a fixed size, relative to its parent.
    pub fn fixed_size(width: Val, height: Val) -> Self {
        Self {
            flex_base: Size::new(width, height),
            ..default()
        }
    }

    /// Returns the style for an item with a minimum size, relative to its
    /// parent.
    ///
    /// The item may grow if more space is available.
    pub fn minimum_size(width: Val, height: Val) -> Self {
        Self {
            flex_base: Size::new(width, height),
            flex_grow: 1.,
            ..default()
        }
    }

    /// Returns the style for an item with a preferred size, relative to its
    /// parent.
    ///
    /// The item may shrink if necessary.
    pub fn preferred_size(width: Val, height: Val) -> Self {
        Self {
            flex_base: Size::new(width, height),
            flex_shrink: 1.,
            ..default()
        }
    }

    /// Returns the style for an item with a preferred size, relative to its
    /// parent.
    ///
    /// The item may shrink if necessary, but not smaller than the given minimum
    /// size.
    pub fn preferred_and_minimum_size(flex_base: Size, min_size: Size) -> Self {
        Self {
            flex_base,
            flex_shrink: 1.,
            min_size,
            ..default()
        }
    }

    /// Sets the given alignment on the `align_self` field.
    pub fn with_alignment(self, align_self: Alignment) -> Self {
        Self { align_self, ..self }
    }

    /// Sets the `preserve_aspect_ratio` boolean to `true`.
    pub fn with_fixed_aspect_ratio(self) -> Self {
        Self {
            preserve_aspect_ratio: true,
            ..self
        }
    }

    /// Adds the given margin to the style.
    pub fn with_margin(self, margin: Size) -> Self {
        Self { margin, ..self }
    }

    /// Adds the given transform to the style.
    pub fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    /// Makes this item not count towards the space taken by the container's
    /// items.
    ///
    /// For more information, see the `occupies_space` field.
    pub fn without_occupying_space(self) -> Self {
        Self {
            occupies_space: false,
            ..self
        }
    }
}

impl Default for FlexItemStyle {
    fn default() -> Self {
        Self {
            align_self: Default::default(),
            flex_base: Default::default(),
            flex_grow: 0.,
            flex_shrink: 0.,
            margin: Default::default(),
            min_size: Default::default(),
            occupies_space: true,
            preserve_aspect_ratio: false,
            transform: Default::default(),
        }
    }
}

/// A leaf item intended to parent non-flex entities.
#[derive(Bundle, Clone, Default)]
pub struct FlexLeafBundle {
    pub flex: FlexItemBundle,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl FlexLeafBundle {
    pub fn from_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexItemBundle::from_style(style),
            ..default()
        }
    }
}

/// A text item to be placed inside a flex container.
///
/// It will use all the available space in the container, and currently, only
/// a single text entity is supported within a container.
#[derive(Bundle, Clone, Default)]
pub struct FlexTextBundle {
    pub flex: Flex,
    pub text: Text2dBundle,
}

impl FlexTextBundle {
    pub fn from_text(text: Text) -> Self {
        Self {
            flex: Flex,
            text: Text2dBundle {
                text,
                transform: Transform::from_scale(Vec3::new(0., 0., 0.)),
                ..default()
            },
        }
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.text.text_anchor = anchor;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Alignment {
    #[default]
    Centered,
    End,
    Start,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: Val,
    pub height: Val,
}

impl Size {
    pub fn all(val: Val) -> Self {
        Self::new(val, val)
    }

    pub fn new(width: Val, height: Val) -> Self {
        Self { width, height }
    }

    pub fn for_direction(&self, direction: FlexDirection) -> Val {
        match direction {
            FlexDirection::Column => self.height,
            FlexDirection::Row => self.width,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Val {
    /// Nada.
    #[default]
    None,
    /// Context-dependent "automatic" value. Will act as `None` in most cases,
    /// unless otherwise specified.
    Auto,
    /// Percentage along the relevant axis. This is a percentage of the width or
    /// height of the parent entity, not the entire window.
    Percent(f32),
    /// Percentage along the cross axis. If the cross axis is defined using
    /// `Val::Percent`, `Val::CrossPercent` can be used to define a related
    /// percentage that maintains aspect ratio.
    CrossPercent(f32),
    /// Percentage along the longest axis of the viewport. This is a percentage
    /// of the width or height of the entire window.
    ///
    /// Note that currently `Vmax` is only supported as long as there is a
    /// direct chain from the `Screen` to the flex item through (nested) flex
    /// containers.
    Vmax(f32),
    /// Percentage along the shortest axis of the viewport. This is a percentage
    /// of the width or height of the entire window.
    ///
    /// Note that currently `Vmin` is only supported as long as there is a
    /// direct chain from the `Screen` to the flex item through (nested) flex
    /// containers.
    Vmin(f32),
}

impl Val {
    pub fn evaluate(&self, axis_scaling: &AxisScaling) -> f32 {
        match self {
            Self::Auto | Self::None => 0.,
            Self::Percent(value) => value * 0.01,
            Self::CrossPercent(value) => axis_scaling.ratio * value * 0.01,
            Self::Vmax(value) => axis_scaling.vmax_scale * value,
            Self::Vmin(value) => axis_scaling.vmin_scale * value,
        }
    }
}

impl Mul<Val> for f32 {
    type Output = Val;

    fn mul(self, rhs: Val) -> Self::Output {
        match rhs {
            Val::None => Val::None,
            Val::Auto => Val::Auto,
            Val::Percent(percentage) => Val::Percent(self * percentage),
            Val::CrossPercent(percentage) => Val::CrossPercent(self * percentage),
            Val::Vmax(percentage) => Val::Vmax(self * percentage),
            Val::Vmin(percentage) => Val::Vmin(self * percentage),
        }
    }
}

#[derive(Clone, Component, Copy, Debug, Default)]
pub struct ComputedPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ComputedPosition {
    pub fn contains(&self, coords: Vec2) -> bool {
        self.x <= coords.x
            && self.y <= coords.y
            && self.x + self.width >= coords.x
            && self.y + self.height >= coords.y
    }

    /// Returns the computed position with the given scale and translation
    /// applied for positioning a child item.
    pub fn transformed(&self, scale: Vec3, translation: Vec3) -> Self {
        let width = self.width * scale.x;
        let height = self.height * scale.y;
        Self {
            x: self.x + (0.5 + translation.x) * self.width - 0.5 * width,
            y: self.y + (0.5 + translation.y) * self.height - 0.5 * height,
            width,
            height,
        }
    }

    pub fn vminmax_scales(&self, screen_width: f32, screen_height: f32) -> VminmaxScales {
        let horizontal_scaling = screen_width / self.width * 0.01;
        let vertical_scaling = screen_height / self.height * 0.01;
        match screen_width > screen_height {
            true => VminmaxScales {
                horizontal: AxisScaling {
                    ratio: self.height / self.width,
                    vmin_scale: horizontal_scaling * screen_height / screen_width,
                    vmax_scale: horizontal_scaling,
                },
                vertical: AxisScaling {
                    ratio: self.width / self.height,
                    vmin_scale: vertical_scaling,
                    vmax_scale: vertical_scaling * screen_width / screen_height,
                },
            },
            false => VminmaxScales {
                horizontal: AxisScaling {
                    ratio: self.height / self.width,
                    vmin_scale: horizontal_scaling,
                    vmax_scale: horizontal_scaling * screen_height / screen_width,
                },
                vertical: AxisScaling {
                    ratio: self.width / self.height,
                    vmin_scale: vertical_scaling * screen_width / screen_height,
                    vmax_scale: vertical_scaling,
                },
            },
        }
    }
}

/// Scales for evaluating the `Vmin` and `Vmax` values.
///
/// Tracks the scales for both axes.
#[derive(Clone, Debug, Default)]
pub struct VminmaxScales {
    pub horizontal: AxisScaling,
    pub vertical: AxisScaling,
}

impl VminmaxScales {
    pub fn scaling_for_direction(&self, direction: FlexDirection) -> AxisScaling {
        match direction {
            FlexDirection::Row => self.horizontal,
            FlexDirection::Column => self.vertical,
        }
    }
}

/// Scales for evaluating the `CrossPercent`, `Vmin`, and `Vmax` values for a
/// single axis.
#[derive(Clone, Copy, Debug)]
pub struct AxisScaling {
    pub ratio: f32,
    pub vmin_scale: f32,
    pub vmax_scale: f32,
}

impl Default for AxisScaling {
    fn default() -> Self {
        Self {
            ratio: 1.,
            vmin_scale: 0.01,
            vmax_scale: 0.01,
        }
    }
}
