use crate::highscores::Highscores;
use crate::resource_bag::ResourceTuple;
use crate::{Game, ResourceBag, Settings};
use bevy::prelude::*;

pub type PropsTuple<'w> = (
    Res<'w, Game>,
    Res<'w, Highscores>,
    ResourceTuple<'w>,
    Res<'w, Settings>,
);

/// Bag of properties for constructing UI components.
pub struct Props<'w> {
    pub game: &'w Game,
    pub highscores: &'w Highscores,
    pub resources: ResourceBag<'w>,
    pub settings: &'w Settings,
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
