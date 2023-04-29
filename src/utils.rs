use bevy::prelude::*;

pub trait SpriteExt {
    fn from_color(color: Color) -> Sprite;
}

impl SpriteExt for Sprite {
    fn from_color(color: Color) -> Sprite {
        Sprite { color, ..default() }
    }
}
