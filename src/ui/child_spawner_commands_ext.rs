use super::Props;
use bevy::prelude::*;

pub struct BundleWithChildren<B>
where
    B: Bundle,
{
    bundle: Option<B>,
    spawn_children: Option<Box<dyn FnOnce(&Props, &mut ChildSpawnerCommands)>>,
}

impl BundleWithChildren<()> {
    pub fn from_closure<C>(spawn_children: C) -> Self
    where
        C: FnOnce(&Props, &mut ChildSpawnerCommands) + 'static,
    {
        Self {
            bundle: None,
            spawn_children: Some(Box::new(spawn_children)),
        }
    }
}

impl<B> From<B> for BundleWithChildren<B>
where
    B: Bundle,
{
    fn from(bundle: B) -> Self {
        Self {
            bundle: Some(bundle),
            spawn_children: None,
        }
    }
}

impl<C> From<C> for BundleWithChildren<()>
where
    C: FnOnce(&Props, &mut ChildSpawnerCommands) + 'static,
{
    fn from(spawn_children: C) -> Self {
        Self::from_closure(spawn_children)
    }
}

impl<B, C> From<(B, C)> for BundleWithChildren<B>
where
    B: Bundle,
    C: FnOnce(&Props, &mut ChildSpawnerCommands) + 'static,
{
    fn from((bundle, spawn_children): (B, C)) -> Self {
        Self {
            bundle: Some(bundle),
            spawn_children: Some(Box::new(spawn_children)),
        }
    }
}

pub trait ChildSpawnerCommandsExt<B>
where
    B: Bundle,
{
    fn spawn_with_children(
        &mut self,
        props: &Props,
        bundle_with_children: impl Into<BundleWithChildren<B>>,
    );
}

impl<B> ChildSpawnerCommandsExt<B> for ChildSpawnerCommands<'_>
where
    B: Bundle,
{
    fn spawn_with_children(
        &mut self,
        props: &Props,
        bundle_with_children: impl Into<BundleWithChildren<B>>,
    ) {
        let BundleWithChildren {
            bundle,
            spawn_children,
        } = bundle_with_children.into();

        if let Some(bundle) = bundle {
            let mut entity_commands = self.spawn(bundle);

            if let Some(spawn_children) = spawn_children {
                entity_commands.with_children(|spawner| spawn_children(props, spawner));
            }
        } else if let Some(spawn_children) = spawn_children {
            spawn_children(props, self);
        }
    }
}

impl<B> ChildSpawnerCommandsExt<B> for Commands<'_, '_>
where
    B: Bundle,
{
    fn spawn_with_children(
        &mut self,
        props: &Props,
        bundle_with_children: impl Into<BundleWithChildren<B>>,
    ) {
        let BundleWithChildren {
            bundle,
            spawn_children,
        } = bundle_with_children.into();

        if let Some(bundle) = bundle {
            let mut entity_commands = self.spawn(bundle);

            if let Some(spawn_children) = spawn_children {
                entity_commands.with_children(|spawner| spawn_children(props, spawner));
            }
        } else {
            unimplemented!("Need a bundle to spawn top-level entities");
        }
    }
}
