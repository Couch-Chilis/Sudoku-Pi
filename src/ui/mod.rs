mod button_bundle;
mod flex_bundles;
mod interaction;
mod layout;

use bevy::prelude::{App, Plugin};
pub use button_bundle::*;
pub use flex_bundles::*;
pub use interaction::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            interaction::mouse_interaction,
            interaction::button_interaction,
            layout::ui_layout_system,
            layout::on_resize_layout,
        ));
    }
}
