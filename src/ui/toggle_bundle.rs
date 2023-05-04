use bevy::prelude::*;

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
