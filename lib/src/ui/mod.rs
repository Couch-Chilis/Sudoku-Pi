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
        app.configure_set(
            PostUpdate,
            LayoutSystem::ApplyLayout.before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            (
                layout::layout_system.in_set(LayoutSystem::ApplyLayout),
                component_animator_system::<FlexItemStyle>.before(LayoutSystem::ApplyLayout),
            ),
        )
        .add_systems(
            Update,
            (
                interaction::keyboard_interaction,
                interaction::pointer_interaction,
                interaction::button_interaction,
            ),
        );
    }
}
