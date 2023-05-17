mod buttons;
mod flex;
mod interaction;
mod layout;
mod toggles;

use bevy::prelude::{App, Plugin};
pub use buttons::*;
pub use flex::*;
pub use interaction::*;
pub use toggles::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            layout::layout_system,
            interaction::mouse_interaction,
            interaction::button_interaction,
        ));
    }
}
