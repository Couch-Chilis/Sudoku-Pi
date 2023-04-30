use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

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
            transform: Default::default(),
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
    /// Padding to keep within the container and around the items.
    pub padding: Size,
}

impl FlexContainerStyle {
    pub fn with_direction(direction: FlexDirection) -> Self {
        Self {
            direction,
            ..default()
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum FlexDirection {
    #[default]
    Column,
    Row,
}

/// A layout bundle based on the flex system, though we're only bothering with
/// the parts relevant to us.
///
/// The main reason for using a custom system instead of Bevy UI is because we
/// need components that can be mixed and matched with non-UI entities, to allow
/// custom transforms on them.
///
/// Flex items should be attached to a renderable entities, rather than
/// inserting the entities as a children of flex items. This also allows a
/// single entity to act as both flex item and container.
#[derive(Bundle, Clone)]
pub struct FlexItemBundle {
    pub flex: Flex,
    pub style: FlexItemStyle,
    pub computed_position: ComputedPosition,
}

impl FlexItemBundle {
    pub fn with_style(style: FlexItemStyle) -> Self {
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

    /// Sets the `occupies_space` boolean to `false`.
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

/// A leaf item intended to parent non-flex entities or to act as a spacer.
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
    /// Returns a "spacer", a flex item whose only purpose is to eat up unused
    /// space, thereby pushing surrounding items to the outer edges of the
    /// container.
    pub fn spacer() -> Self {
        Self::with_style(FlexItemStyle::available_size())
    }

    pub fn with_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexItemBundle::with_style(style),
            ..default()
        }
    }
}

#[derive(Clone, Copy, Default)]
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

    pub fn width(&self) -> Val {
        self.width
    }

    pub fn height(&self) -> Val {
        self.height
    }
}

#[derive(Clone, Copy, Default)]
pub enum Val {
    /// Nada.
    #[default]
    None,
    /// Percentage along the relevant axis. This is a percentage of the width or
    /// height of the parent entity, not the entire window.
    Percent(f32),
    /// Percentage along the shortest axis. This is a percentage of the width or
    /// height of the parent entity, not the entire window.
    ///
    /// Note that currently `Vmin` is only supported as long as there is a
    /// direct chain from the `Screen` to the flex item through (nested) flex
    /// containers.
    Vmin(f32),
}

impl Val {
    pub fn evaluate(&self, vmin_scale: f32) -> f32 {
        match self {
            Self::None => 0.,
            Self::Percent(value) => 0.01 * value,
            Self::Vmin(value) => vmin_scale * value,
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

    pub fn vmin_scales(&self) -> Vec2 {
        match self.width < self.height {
            true => Vec2::new(0.01, 0.01 * self.width / self.height),
            false => Vec2::new(0.01 * self.height / self.width, 0.01),
        }
    }
}
