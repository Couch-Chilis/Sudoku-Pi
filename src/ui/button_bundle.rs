use super::{FlexItemBundle, FlexItemStyle, Interaction};
use crate::{constants::NORMAL_BUTTON, utils::SpriteExt};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

/// Marker for buttons.
#[derive(Clone, Component)]
pub struct Button;

/// Marks a button as secondary.
#[derive(Clone, Component)]
pub struct Secondary;

/// A UI button with text that is also a flex item.
#[derive(Bundle, Clone)]
pub struct ButtonBundle {
    pub button: Button,
    pub flex: FlexItemBundle,
    pub interaction: Interaction,

    pub background: Sprite,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl ButtonBundle {
    pub fn with_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexItemBundle::with_style(style),
            background: Sprite::from_color(NORMAL_BUTTON),
            ..default()
        }
    }
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self {
            button: Button,
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
