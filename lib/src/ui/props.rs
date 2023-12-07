use crate::highscores::Highscores;
use crate::resource_bag::ResourceTuple;
use crate::{Game, ResourceBag, Settings};
use bevy::prelude::*;

pub type PropsTuple<'a> = (
    Res<'a, Game>,
    Res<'a, Highscores>,
    ResourceTuple<'a>,
    Res<'a, Settings>,
);

/// Bag of properties for constructing UI components.
pub struct Props<'a> {
    pub game: &'a Game,
    pub highscores: &'a Highscores,
    pub resources: ResourceBag<'a>,
    pub settings: &'a Settings,
}

impl<'a> Props<'a> {
    pub fn from_tuple((game, highscores, resources, settings): &'a PropsTuple<'a>) -> Self {
        Self {
            game,
            highscores,
            resources: ResourceBag::from_tuple(resources),
            settings,
        }
    }
}
