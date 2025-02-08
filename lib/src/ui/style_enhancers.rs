use super::flex::*;
use crate::resource_bag::ResourceBag;

/// Trait that defines enhancers for [FlexContainerBundle].
pub trait FlexContainerBundleEnhancer: Sized {
    fn enhance(self, bundle: &mut FlexContainerBundle);
}

impl<T> FlexContainerBundleEnhancer for T
where
    T: FnOnce(&mut FlexContainerBundle) + Sized,
{
    fn enhance(self, bundle: &mut FlexContainerBundle) {
        self(bundle)
    }
}

impl FlexContainerBundleEnhancer for () {
    fn enhance(self, _bundle: &mut FlexContainerBundle) {}
}

impl<T, U> FlexContainerBundleEnhancer for (T, U)
where
    T: FlexContainerBundleEnhancer,
    U: FlexContainerBundleEnhancer,
{
    fn enhance(self, bundle: &mut FlexContainerBundle) {
        self.0.enhance(bundle);
        self.1.enhance(bundle);
    }
}

/// Trait that defines enhancers for [FlexItemStyle].
pub trait FlexItemStyleEnhancer: Sized {
    fn enhance(self, style: &mut FlexItemStyle);
}

impl<T> FlexItemStyleEnhancer for T
where
    T: FnOnce(&mut FlexItemStyle) + Sized,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self(style)
    }
}

impl FlexItemStyleEnhancer for () {
    fn enhance(self, _style: &mut FlexItemStyle) {}
}

impl<T, U> FlexItemStyleEnhancer for (T, U)
where
    T: FlexItemStyleEnhancer,
    U: FlexItemStyleEnhancer,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
    }
}

impl<T, U, V> FlexItemStyleEnhancer for (T, U, V)
where
    T: FlexItemStyleEnhancer,
    U: FlexItemStyleEnhancer,
    V: FlexItemStyleEnhancer,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
        self.2.enhance(style);
    }
}

impl<T, U, V, W> FlexItemStyleEnhancer for (T, U, V, W)
where
    T: FlexItemStyleEnhancer,
    U: FlexItemStyleEnhancer,
    V: FlexItemStyleEnhancer,
    W: FlexItemStyleEnhancer,
{
    fn enhance(self, style: &mut FlexItemStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
        self.2.enhance(style);
        self.3.enhance(style);
    }
}

/// Trait that defines enhancers for [Text2dBounde].
pub trait TextEnhancer: Sized {
    fn enhance(self, style: &mut FlexTextStyle, resources: &ResourceBag);
}

impl<T> TextEnhancer for T
where
    T: FnOnce(&mut FlexTextStyle, &ResourceBag) + Sized,
{
    fn enhance(self, style: &mut FlexTextStyle, resources: &ResourceBag) {
        self(style, resources)
    }
}

impl TextEnhancer for () {
    fn enhance(self, _style: &mut FlexTextStyle, _resources: &ResourceBag) {}
}

impl<T, U> TextEnhancer for (T, U)
where
    T: TextEnhancer,
    U: TextEnhancer,
{
    fn enhance(self, style: &mut FlexTextStyle, resources: &ResourceBag) {
        self.0.enhance(style, resources);
        self.1.enhance(style, resources);
    }
}

impl<T, U, V> TextEnhancer for (T, U, V)
where
    T: TextEnhancer,
    U: TextEnhancer,
    V: TextEnhancer,
{
    fn enhance(self, style: &mut FlexTextStyle, resources: &ResourceBag) {
        self.0.enhance(style, resources);
        self.1.enhance(style, resources);
        self.2.enhance(style, resources);
    }
}

impl<T, U, V, W> TextEnhancer for (T, U, V, W)
where
    T: TextEnhancer,
    U: TextEnhancer,
    V: TextEnhancer,
    W: TextEnhancer,
{
    fn enhance(self, style: &mut FlexTextStyle, resources: &ResourceBag) {
        self.0.enhance(style, resources);
        self.1.enhance(style, resources);
        self.2.enhance(style, resources);
        self.3.enhance(style, resources);
    }
}
