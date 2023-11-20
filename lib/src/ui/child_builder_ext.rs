use bevy::prelude::*;

pub trait ChildBuilderExt {
    fn spawn_with_children(
        &mut self,
        bundle_with_children: (impl Bundle, impl FnOnce(&mut ChildBuilder)),
    );
}

impl<'w, 's, 'a> ChildBuilderExt for ChildBuilder<'w, 's, 'a> {
    fn spawn_with_children(
        &mut self,
        (bundle, spawn_children): (impl Bundle, impl FnOnce(&mut ChildBuilder)),
    ) {
        self.spawn(bundle).with_children(spawn_children);
    }
}
