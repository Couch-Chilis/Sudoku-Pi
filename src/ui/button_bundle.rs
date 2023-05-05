use super::{FlexItemBundle, FlexItemStyle, Interaction};
use crate::constants::COLOR_BUTTON_BACKGROUND;
use crate::utils::{SpriteExt, TransformExt};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

/// Marker for buttons.
#[derive(Clone, Component)]
pub struct Button;

/// Marks a button as secondary.
#[derive(Clone, Component, Default)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Ternary,
}

/// Marker for button borders.
#[derive(Clone, Component)]
pub struct Border;

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
    pub fn from_style(style: FlexItemStyle) -> Self {
        Self {
            flex: FlexItemBundle::from_style(style),
            background: Sprite::from_color(COLOR_BUTTON_BACKGROUND),
            transform: Transform::default_2d(),
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
