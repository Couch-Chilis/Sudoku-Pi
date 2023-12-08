use super::flex::*;
use crate::resource_bag::ResourceBag;
use bevy::text::Text;

/// Trait that defines enhancers for [FlexContainerStyle].
pub trait FlexContainerStyleEnhancer: Sized {
    fn enhance(self, style: &mut FlexContainerStyle);
}

impl<T> FlexContainerStyleEnhancer for T
where
    T: FnOnce(&mut FlexContainerStyle) + Sized,
{
    fn enhance(self, style: &mut FlexContainerStyle) {
        self(style)
    }
}

impl FlexContainerStyleEnhancer for () {
    fn enhance(self, _style: &mut FlexContainerStyle) {}
}

impl<T, U> FlexContainerStyleEnhancer for (T, U)
where
    T: FlexContainerStyleEnhancer,
    U: FlexContainerStyleEnhancer,
{
    fn enhance(self, style: &mut FlexContainerStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
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

/// Trait that defines enhancers for [TextStyle].
pub trait TextEnhancer: Sized {
    fn enhance(self, text: &mut Text, resources: &ResourceBag);
}

impl<T> TextEnhancer for T
where
    T: FnOnce(&mut Text, &ResourceBag) + Sized,
{
    fn enhance(self, text: &mut Text, resources: &ResourceBag) {
        self(text, resources)
    }
}

impl TextEnhancer for () {
    fn enhance(self, _text: &mut Text, _resources: &ResourceBag) {}
}

impl<T, U> TextEnhancer for (T, U)
where
    T: TextEnhancer,
    U: TextEnhancer,
{
    fn enhance(self, text: &mut Text, resources: &ResourceBag) {
        self.0.enhance(text, resources);
        self.1.enhance(text, resources);
    }
}

impl<T, U, V> TextEnhancer for (T, U, V)
where
    T: TextEnhancer,
    U: TextEnhancer,
    V: TextEnhancer,
{
    fn enhance(self, text: &mut Text, resources: &ResourceBag) {
        self.0.enhance(text, resources);
        self.1.enhance(text, resources);
        self.2.enhance(text, resources);
    }
}

impl<T, U, V, W> TextEnhancer for (T, U, V, W)
where
    T: TextEnhancer,
    U: TextEnhancer,
    V: TextEnhancer,
    W: TextEnhancer,
{
    fn enhance(self, text: &mut Text, resources: &ResourceBag) {
        self.0.enhance(text, resources);
        self.1.enhance(text, resources);
        self.2.enhance(text, resources);
        self.3.enhance(text, resources);
    }
}
