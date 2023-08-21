mod buttons;
mod flex;
mod interaction;
mod layout;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_tweening::component_animator_system;
pub use buttons::*;
pub use flex::*;
pub use interaction::*;

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
                interaction::reset_initial_selection_on_screen_change,
                interaction::keyboard_interaction
                    .after(interaction::reset_initial_selection_on_screen_change),
                interaction::pointer_interaction
                    .after(interaction::reset_initial_selection_on_screen_change),
                interaction::button_interaction
                    .after(interaction::keyboard_interaction)
                    .after(interaction::pointer_interaction),
            ),
        );
    }
}
