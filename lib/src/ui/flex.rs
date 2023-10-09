#![allow(dead_code)]

use crate::{utils::*, ScreenInteraction, ScreenState};
use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
use smallvec::SmallVec;
use std::ops::{Add, Mul, Sub};

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
        Self::new(item_style, FlexContainerStyle::default())
    }

    pub fn new(item_style: FlexItemStyle, container_style: FlexContainerStyle) -> Self {
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

    pub fn with_background_color(self, color: Color) -> Self {
        Self {
            container: self.container.with_background_color(color),
            item: self.item,
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
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for FlexContainerBundle {
    fn default() -> Self {
        Self {
            style: FlexContainerStyle::default(),
            background: Sprite::from_color(Color::NONE),
            transform: Transform::default_2d(),
            global_transform: Default::default(),
            texture: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}

impl FlexContainerBundle {
    pub fn with_background_color(self, color: Color) -> Self {
        Self {
            background: Sprite::from_color(color),
            ..self
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
    pub padding: Sides,
}

impl FlexContainerStyle {
    pub fn column() -> Self {
        Self::default()
    }

    pub fn row() -> Self {
        Self {
            direction: FlexDirection::Row,
            ..default()
        }
    }

    pub fn with_gap(self, gap: Val) -> Self {
        Self { gap, ..self }
    }

    pub fn with_padding(self, padding: Sides) -> Self {
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
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
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
    pub computed_position: ComputedPosition,
}

impl FlexTextBundle {
    pub fn from_text(text: Text) -> Self {
        Self {
            text: Text2dBundle {
                text,
                transform: Transform::from_scale(Vec3::new(0., 0., 0.)),
                ..default()
            },
            ..default()
        }
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.text.text_anchor = anchor;
        self
    }

    pub fn with_bounds(mut self, bounds: Text2dBounds) -> Self {
        self.text.text_2d_bounds = bounds;
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

#[derive(Clone, Debug, Default)]
pub struct Size {
    pub width: Val,
    pub height: Val,
}

impl Size {
    pub fn all(val: Val) -> Self {
        Self::new(val.clone(), val)
    }

    pub fn new(width: Val, height: Val) -> Self {
        Self { width, height }
    }

    pub fn for_direction(&self, direction: FlexDirection) -> Val {
        match direction {
            FlexDirection::Column => self.height.clone(),
            FlexDirection::Row => self.width.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sides {
    pub top: Val,
    pub right: Val,
    pub bottom: Val,
    pub left: Val,
}

impl Sides {
    pub fn all(val: Val) -> Self {
        Self::new(val.clone(), val)
    }

    pub fn bottom(bottom: Val) -> Self {
        Self {
            top: Val::None,
            right: Val::None,
            bottom,
            left: Val::None,
        }
    }

    pub fn horizontal(horizontal: Val) -> Self {
        Self::new(horizontal, Val::None)
    }

    pub fn left(left: Val) -> Self {
        Self {
            top: Val::None,
            right: Val::None,
            bottom: Val::None,
            left,
        }
    }

    pub fn new(horizontal: Val, vertical: Val) -> Self {
        Self {
            top: vertical.clone(),
            right: horizontal.clone(),
            bottom: vertical,
            left: horizontal,
        }
    }

    pub fn right(right: Val) -> Self {
        Self {
            top: Val::None,
            right,
            bottom: Val::None,
            left: Val::None,
        }
    }

    pub fn top(top: Val) -> Self {
        Self {
            top,
            right: Val::None,
            bottom: Val::None,
            left: Val::None,
        }
    }

    pub fn vertical(vertical: Val) -> Self {
        Self::new(Val::None, vertical)
    }

    pub fn before_for_direction(&self, direction: FlexDirection) -> Val {
        match direction {
            FlexDirection::Column => self.top.clone(),
            FlexDirection::Row => self.left.clone(),
        }
    }

    pub fn after_for_direction(&self, direction: FlexDirection) -> Val {
        match direction {
            FlexDirection::Column => self.bottom.clone(),
            FlexDirection::Row => self.right.clone(),
        }
    }

    pub fn with_top(self, top: Val) -> Self {
        Self { top, ..self }
    }

    pub fn with_right(self, right: Val) -> Self {
        Self { right, ..self }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Val {
    /// Nada.
    #[default]
    None,
    /// Context-dependent "automatic" value. Will act as `None` in most cases,
    /// unless otherwise specified.
    Auto,
    /// An exact pixel value, measured in logical pixels.
    Pixel(i32),
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
    /// Calculates a value from one or more others.
    Calc(Box<Expr>),
}

impl Val {
    pub fn evaluate(&self, axis_scaling: &AxisScaling) -> f32 {
        match self {
            Self::Auto | Self::None => 0.,
            Self::Pixel(value) => axis_scaling.pixel_scale * (*value as f32),
            Self::Percent(value) => value * 0.01,
            Self::CrossPercent(value) => axis_scaling.axis_ratio * value * 0.01,
            Self::Vmax(value) => axis_scaling.vmax_scale * value,
            Self::Vmin(value) => axis_scaling.vmin_scale * value,
            Self::Calc(expr) => expr.evaluate(axis_scaling),
        }
    }
}

impl Mul<Val> for f32 {
    type Output = Val;

    fn mul(self, rhs: Val) -> Self::Output {
        match rhs {
            Val::None => Val::None,
            Val::Auto => Val::Auto,
            Val::Pixel(value) => Val::Pixel((self * value as f32) as i32),
            Val::Percent(percentage) => Val::Percent(self * percentage),
            Val::CrossPercent(percentage) => Val::CrossPercent(self * percentage),
            Val::Vmax(percentage) => Val::Vmax(self * percentage),
            Val::Vmin(percentage) => Val::Vmin(self * percentage),
            Val::Calc(expr) => Val::Calc(Box::new(self * *expr)),
        }
    }
}

impl Add<Val> for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        match (self, rhs) {
            (lhs, Val::None) => lhs,
            (Self::None | Self::Auto, rhs) => rhs,
            (Self::Pixel(value), Val::Pixel(rhs)) => Self::Pixel(value + rhs),
            (Self::Pixel(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Pixel(value),
                operator: Operator::Plus,
                right: rhs,
            })),
            (Self::Percent(value), Val::Percent(rhs)) => Self::Percent(value + rhs),
            (Self::Percent(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Percent(value),
                operator: Operator::Plus,
                right: rhs,
            })),
            (Self::CrossPercent(value), Val::CrossPercent(rhs)) => Self::CrossPercent(value + rhs),
            (Self::CrossPercent(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::CrossPercent(value),
                operator: Operator::Plus,
                right: rhs,
            })),
            (Self::Vmax(value), Val::Vmax(rhs)) => Self::Vmax(value + rhs),
            (Self::Vmax(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Vmax(value),
                operator: Operator::Plus,
                right: rhs,
            })),
            (Self::Vmin(value), Val::Vmin(rhs)) => Self::Vmin(value + rhs),
            (Self::Vmin(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Vmin(value),
                operator: Operator::Plus,
                right: rhs,
            })),
            (Self::Calc(expr), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Calc(expr),
                operator: Operator::Plus,
                right: rhs,
            })),
        }
    }
}

impl Sub<Val> for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Self::Output {
        match (self, rhs) {
            (lhs, Val::None) => lhs,
            (Self::None, _) => Self::None,
            (Self::Auto, Val::Auto) => Self::None,
            (Self::Auto, rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Auto,
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::Pixel(value), Val::Pixel(rhs)) => Self::Pixel(value - rhs),
            (Self::Pixel(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Pixel(value),
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::Percent(value), Val::Percent(rhs)) => Self::Percent(value - rhs),
            (Self::Percent(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Percent(value),
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::CrossPercent(value), Val::CrossPercent(rhs)) => Self::CrossPercent(value - rhs),
            (Self::CrossPercent(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::CrossPercent(value),
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::Vmax(value), Val::Vmax(rhs)) => Self::Vmax(value - rhs),
            (Self::Vmax(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Vmax(value),
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::Vmin(value), Val::Vmin(rhs)) => Self::Vmin(value - rhs),
            (Self::Vmin(value), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Vmin(value),
                operator: Operator::Minus,
                right: rhs,
            })),
            (Self::Calc(expr), rhs) => Val::Calc(Box::new(Expr::Binary {
                left: Self::Calc(expr),
                operator: Operator::Minus,
                right: rhs,
            })),
        }
    }
}

#[derive(Clone, Component, Debug, Default)]
pub struct ComputedPosition {
    pub x: f32,
    pub y: f32,
    pub screens: SmallVec<[ScreenState; 4]>,
    pub width: f32,
    pub height: f32,
}

impl ComputedPosition {
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + 0.5 * self.width, self.y + 0.5 * self.height)
    }

    pub fn contains(&self, coords: Vec2) -> bool {
        self.x <= coords.x
            && self.y <= coords.y
            && self.x + self.width >= coords.x
            && self.y + self.height >= coords.y
    }

    /// Returns the computed position with the given scale and translation
    /// applied for positioning a child item.
    pub fn transformed(&self, scale: Vec3, translation: Vec3) -> Self {
        self.transformed_with_screens(scale, translation, self.screens.clone())
    }

    /// Same as `transformed()`, but also sets the `screens` field.
    pub fn transformed_with_screen_interaction(
        &self,
        scale: Vec3,
        translation: Vec3,
        screen_interaction: Option<&ScreenInteraction>,
    ) -> Self {
        self.transformed_with_screens(
            scale,
            translation,
            screen_interaction
                .map_or(&self.screens, |interaction| &interaction.screens)
                .clone(),
        )
    }

    fn transformed_with_screens(
        &self,
        scale: Vec3,
        translation: Vec3,
        screens: SmallVec<[ScreenState; 4]>,
    ) -> Self {
        let width = self.width * scale.x;
        let height = self.height * scale.y;
        Self {
            x: self.x + (0.5 + translation.x) * self.width - 0.5 * width,
            y: self.y + (0.5 + translation.y) * self.height - 0.5 * height,
            screens,
            width,
            height,
        }
    }

    pub fn axis_scales(&self, screen_width: f32, screen_height: f32) -> AxesScaling {
        let horizontal_scaling = screen_width / self.width * 0.01;
        let vertical_scaling = screen_height / self.height * 0.01;
        match screen_width > screen_height {
            true => AxesScaling {
                horizontal: AxisScaling {
                    axis_ratio: self.height / self.width,
                    pixel_scale: 1. / self.width,
                    vmin_scale: horizontal_scaling * screen_height / screen_width,
                    vmax_scale: horizontal_scaling,
                },
                vertical: AxisScaling {
                    axis_ratio: self.width / self.height,
                    pixel_scale: 1. / self.height,
                    vmin_scale: vertical_scaling,
                    vmax_scale: vertical_scaling * screen_width / screen_height,
                },
            },
            false => AxesScaling {
                horizontal: AxisScaling {
                    axis_ratio: self.height / self.width,
                    pixel_scale: 1. / self.width,
                    vmin_scale: horizontal_scaling,
                    vmax_scale: horizontal_scaling * screen_height / screen_width,
                },
                vertical: AxisScaling {
                    axis_ratio: self.width / self.height,
                    pixel_scale: 1. / self.height,
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
pub struct AxesScaling {
    pub horizontal: AxisScaling,
    pub vertical: AxisScaling,
}

impl AxesScaling {
    pub fn scaling_for_direction(&self, direction: FlexDirection) -> AxisScaling {
        match direction {
            FlexDirection::Row => self.horizontal,
            FlexDirection::Column => self.vertical,
        }
    }
}

/// Scales for evaluating various `Val` values for a single axis.
#[derive(Clone, Copy, Debug)]
pub struct AxisScaling {
    /// Ratio for converting a value along one axis to a value on the cross
    /// axis.
    pub axis_ratio: f32,

    /// Scale for converting a pixel value.
    pub pixel_scale: f32,

    /// Scale for converting a `Vmin` value.
    pub vmin_scale: f32,

    /// Scale for converting a `Vmax` value.
    pub vmax_scale: f32,
}

impl Default for AxisScaling {
    fn default() -> Self {
        Self {
            axis_ratio: 1.,
            pixel_scale: 0.01,
            vmin_scale: 0.01,
            vmax_scale: 0.01,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Val,
        operator: Operator,
        right: Val,
    },
}

impl Expr {
    pub fn evaluate(&self, axis_scaling: &AxisScaling) -> f32 {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => match operator {
                Operator::Minus => left.evaluate(axis_scaling) - right.evaluate(axis_scaling),
                Operator::Plus => left.evaluate(axis_scaling) + right.evaluate(axis_scaling),
            },
        }
    }
}

impl Mul<Expr> for f32 {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        match rhs {
            Expr::Binary {
                left,
                operator,
                right,
            } => Expr::Binary {
                left: self * left,
                operator,
                right: self * right,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Minus,
    Plus,
}
