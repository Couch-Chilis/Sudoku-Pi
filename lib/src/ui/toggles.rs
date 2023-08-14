use super::flex::*;
use crate::{constants::*, utils::*, Images};
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

pub struct ToggleBuilder<'a> {
    images: &'a Images,
}

impl<'a> ToggleBuilder<'a> {
    pub fn new(images: &'a Images) -> Self {
        Self { images }
    }

    pub fn build_with_marker(
        &mut self,
        parent: &mut ChildBuilder,
        marker: impl Bundle,
        enabled: bool,
    ) {
        parent
            .spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
                Val::CrossPercent(100.),
                Val::Percent(100.),
            )))
            .with_children(|icon_container| {
                /*icon_container.spawn(MaterialMesh2dBundle {
                    mesh: self.meshes.add(shape::Circle::new(0.3).into()).into(),
                    material: self
                        .materials
                        .add(ColorMaterial::from(COLOR_BUTTON_BACKGROUND)),
                    transform: Transform::default_2d(),
                    ..default()
                });

                icon_container.spawn(MaterialMesh2dBundle {
                    mesh: self.meshes.add(shape::Circle::new(0.25).into()).into(),
                    material: self.materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                    ..default()
                });

                icon_container.spawn((
                    Toggle,
                    marker,
                    MaterialMesh2dBundle {
                        mesh: self.meshes.add(shape::Circle::new(0.21).into()).into(),
                        material: self.materials.add(ColorMaterial::from(if enabled {
                            COLOR_TOGGLE_ON
                        } else {
                            COLOR_TOGGLE_OFF
                        })),
                        transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                        ..default()
                    },
                ));*/
            });
    }
}
