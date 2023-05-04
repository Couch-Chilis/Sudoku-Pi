use super::SettingsToggle;
use crate::{constants::*, settings::Settings, ui::*, Fonts};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct ToggleBuilder<'a> {
    container_style: FlexItemStyle,
    text_style: TextStyle,
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<ColorMaterial>,
}

impl<'a> ToggleBuilder<'a> {
    pub fn new(
        fonts: &Fonts,
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<ColorMaterial>,
    ) -> Self {
        let container_style = FlexItemStyle {
            flex_base: Size::new(Val::Vmin(80.), Val::Vmin(14.)),
            margin: Size::all(Val::Vmin(2.)),
            ..default()
        };

        let text_style = TextStyle {
            font: fonts.menu.clone(),
            font_size: 60.,
            color: COLOR_SECONDARY_BUTTON_TEXT,
        };

        Self {
            container_style,
            text_style,
            meshes,
            materials,
        }
    }

    pub fn add_with_text_and_action(
        &mut self,
        parent: &mut ChildBuilder,
        settings: &Settings,
        text: &str,
        toggle: SettingsToggle,
    ) {
        parent
            .spawn((
                ToggleContainer,
                Interaction::None,
                FlexBundle::new(
                    FlexContainerStyle::row().with_padding(Size::all(Val::Vmin(10.))),
                    self.container_style.clone(),
                ),
                toggle,
            ))
            .with_children(|toggle_container| {
                toggle_container
                    .spawn(FlexLeafBundle::from_style(FlexItemStyle::available_size()))
                    .with_children(|label_container| {
                        label_container.spawn(Text2dBundle {
                            text: Text::from_section(text, self.text_style.clone()),
                            transform: Transform {
                                scale: Vec3::new(0.0015, 0.01, 1.),
                                translation: Vec3::new(0., 0., 1.),
                                ..default()
                            },
                            ..default()
                        });
                    });

                toggle_container
                    .spawn(FlexLeafBundle::from_style(FlexItemStyle::fixed_size(
                        Val::Vmin(100.),
                        Val::Vmin(100.),
                    )))
                    .with_children(|icon_container| {
                        icon_container.spawn(MaterialMesh2dBundle {
                            mesh: self.meshes.add(shape::Circle::new(0.5).into()).into(),
                            material: self
                                .materials
                                .add(ColorMaterial::from(COLOR_BUTTON_BACKGROUND)),
                            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                            ..default()
                        });

                        icon_container.spawn((
                            Toggle,
                            toggle,
                            MaterialMesh2dBundle {
                                mesh: self.meshes.add(shape::Circle::new(0.45).into()).into(),
                                material: self.materials.add(ColorMaterial::from(
                                    if toggle.is_enabled(settings) {
                                        COLOR_TOGGLE_ON
                                    } else {
                                        COLOR_TOGGLE_OFF
                                    },
                                )),
                                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                                ..default()
                            },
                        ));
                    });
            });
    }
}
