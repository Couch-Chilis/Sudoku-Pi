use super::{FlexItemBundle, FlexItemStyle, Interaction};
use crate::{constants::COLOR_BUTTON_BACKGROUND, utils::SpriteExt};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

/// Marker for toggle.
#[derive(Clone, Component)]
pub struct Toggle;

/// Marker for toggle container. The container usually contains a text label and
/// the toggle itself.
#[derive(Clone, Component)]
pub struct ToggleContainer;

/// Marks a toggle as enabled.
#[derive(Clone, Component)]
pub struct Enabled;

/// A UI toggle that is also a flex item.
#[derive(Bundle, Clone)]
pub struct ToggleBundle {
    pub toggle: Toggle,
    pub flex: FlexItemBundle,
    pub interaction: Interaction,

    pub background: Sprite,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl ToggleBundle {
    pub fn with_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexItemBundle::from_style(style),
            background: Sprite::from_color(COLOR_BUTTON_BACKGROUND),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        }
    }
}

impl Default for ToggleBundle {
    fn default() -> Self {
        Self {
            toggle: Toggle,
            flex: Default::default(),
            interaction: Default::default(),
            background: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
