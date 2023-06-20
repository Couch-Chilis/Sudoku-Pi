mod buttons;
mod flex;
mod interaction;
mod layout;
mod toggles;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_tweening::component_animator_system;
pub use buttons::*;
pub use flex::*;
pub use interaction::*;
pub use toggles::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum LayoutSystem {
    ApplyLayout,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(LayoutSystem::ApplyLayout.before(TransformSystem::TransformPropagate))
            .add_system(layout::layout_system.in_set(LayoutSystem::ApplyLayout))
            .add_system(
                component_animator_system::<FlexItemStyle>.before(LayoutSystem::ApplyLayout),
            )
            .add_systems((
                interaction::keyboard_interaction,
                interaction::mouse_interaction,
                interaction::touch_interaction,
                interaction::button_interaction,
            ));
    }
}