use crate::resource_bag::ResourceTuple;
use crate::{Game, ResourceBag, Settings};
use bevy::prelude::*;

pub type PropsTuple<'a> = (Res<'a, Game>, ResourceTuple<'a>, Res<'a, Settings>);

/// Bag of properties for constructing UI components.
pub struct Props<'a> {
    pub game: &'a Game,
    pub resources: ResourceBag<'a>,
    pub settings: &'a Settings,
}

impl<'a> Props<'a> {
    pub fn from_tuple((game, resources, settings): &'a PropsTuple<'a>) -> Self {
        Self {
            game,
            resources: ResourceBag::from_tuple(resources),
            settings,
        }
    }
}
