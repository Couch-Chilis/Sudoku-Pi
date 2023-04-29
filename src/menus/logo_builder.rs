use crate::ui::*;
use bevy::prelude::*;

pub fn build_logo(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent
        .spawn(FlexLeafBundle::with_style(FlexItemStyle {
            flex_base: Size::new(Val::Vmin(29.), Val::Vmin(60.)),
            flex_shrink: 1.,
            margin: Size::all(Val::Vmin(15.)),
            preserve_aspect_ratio: true,
            ..default()
        }))
        .with_children(|flex| {
            flex.spawn(SpriteBundle {
                texture: asset_server.load("logo.png"),
                transform: Transform {
                    translation: Vec3::new(0., 0., 3.),
                    scale: Vec3::new(1. / 241., 1. / 513., 1.),
                    ..default()
                },
                ..default()
            });
        });
}
