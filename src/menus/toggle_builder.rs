use crate::{constants::*, ui::*, Fonts};
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
            flex_base: Size::new(Val::Vmin(60.), Val::Vmin(14.)),
            margin: Size::all(Val::Vmin(2.)),
            ..default()
        };

        let text_style = TextStyle {
            font: fonts.menu.clone(),
            font_size: 60.,
            color: COLOR_BUTTON_TEXT,
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
        text: &str,
        action: impl Component,
    ) {
        parent
            .spawn((
                ToggleContainer,
                FlexBundle::new(
                    FlexContainerStyle::row().with_padding(Size::all(Val::Vmin(10.))),
                    self.container_style.clone(),
                ),
                action,
            ))
            .with_children(|button| {
                button.spawn(Text2dBundle {
                    text: Text::from_section(text, self.text_style.clone()),
                    transform: Transform {
                        scale: Vec3::new(0.002, 0.01, 1.),
                        translation: Vec3::new(0., 0., 1.),
                        ..default()
                    },
                    ..default()
                });

                button.spawn(MaterialMesh2dBundle {
                    mesh: self.meshes.add(shape::Circle::new(0.5).into()).into(),
                    material: self
                        .materials
                        .add(ColorMaterial::from(COLOR_BUTTON_BACKGROUND)),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                    ..default()
                });
            });
    }
}
