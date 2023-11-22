use super::flex::*;
use bevy::text::TextStyle;

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
pub trait TextStyleEnhancer: Sized {
    fn enhance(self, style: &mut TextStyle);
}

impl<T> TextStyleEnhancer for T
where
    T: FnOnce(&mut TextStyle) + Sized,
{
    fn enhance(self, style: &mut TextStyle) {
        self(style)
    }
}

impl TextStyleEnhancer for () {
    fn enhance(self, _style: &mut TextStyle) {}
}

impl<T, U> TextStyleEnhancer for (T, U)
where
    T: TextStyleEnhancer,
    U: TextStyleEnhancer,
{
    fn enhance(self, style: &mut TextStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
    }
}

impl<T, U, V> TextStyleEnhancer for (T, U, V)
where
    T: TextStyleEnhancer,
    U: TextStyleEnhancer,
    V: TextStyleEnhancer,
{
    fn enhance(self, style: &mut TextStyle) {
        self.0.enhance(style);
        self.1.enhance(style);
        self.2.enhance(style);
    }
}
