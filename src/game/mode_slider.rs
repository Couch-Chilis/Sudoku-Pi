use crate::{constants::*, ui::*, Fonts};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor, sprite::MaterialMesh2dBundle};

pub fn build_mode_slider(
    parent: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
) {
    parent.with_children(|parent| {
        parent
            .spawn(FlexBundle::new(
                FlexContainerStyle::row().with_padding(Size::new(Val::None, Val::Percent(25.))),
                FlexItemStyle {
                    flex_base: Size::new(Val::Vmin(90.), Val::Vmin(9.)),
                    flex_grow: 2.,
                    margin: Size::all(Val::Vmin(4.5)),
                    ..default()
                },
            ))
            .with_children(|row| build_items(row, meshes, materials, fonts));
    });
}

fn build_items(
    row: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    fonts: &Fonts,
) {
    let text_style = TextStyle {
        font: fonts.medium.clone(),
        font_size: 60.,
        color: COLOR_SECONDARY_BUTTON_TEXT,
    };

    // This is the "enabled" circle that will jump between toggles.
    row.spawn(FlexLeafBundle::from_style(
        FlexItemStyle::fixed_size(Val::CrossPercent(100.), Val::Percent(100.))
            .with_transform(Transform::from_translation(Vec3::new(0., 0., 2.)))
            .without_occupying_space(),
    ))
    .with_children(|circle_container| {
        circle_container.spawn((
            Toggle,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(0.25).into()).into(),
                material: materials.add(ColorMaterial::from(COLOR_TOGGLE_ON)),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..default()
            },
        ));
    });

    let mut toggle_builder = ToggleBuilder::new(meshes, materials);
    toggle_builder.build_with_marker(row, (), false);

    row.spawn(FlexBundle::from_item_style(
        FlexItemStyle::minimum_size(Val::Vmin(25.), Val::Percent(100.))
            .with_transform(Transform::from_translation(Vec3::new(0., 0., 3.))),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Normal", text_style.clone()))
                .with_anchor(Anchor::CenterLeft),
        );
    });

    row.spawn(FlexBundle::from_item_style(
        FlexItemStyle::minimum_size(Val::Vmin(25.), Val::Percent(100.))
            .with_transform(Transform::from_translation(Vec3::new(0., 0., 3.))),
    ))
    .with_children(|label_container| {
        label_container.spawn(
            FlexTextBundle::from_text(Text::from_section("Notes", text_style))
                .with_anchor(Anchor::CenterRight),
        );
    });

    toggle_builder.build_with_marker(row, (), false);
}
