use crate::ZoomFactor;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    PressedMovement,
    Press,
    Release,
}

#[cfg(target_os = "ios")]
pub type PointerQuery<'w, 's, 'a> = TouchQuery<'w, 's, 'a>;

#[cfg(not(target_os = "ios"))]
pub type PointerQuery<'w, 's, 'a> = MouseQuery<'w, 's, 'a>;

pub trait PointerQueryExt {
    fn get_changed_hover(&self) -> Option<Vec2> {
        None
    }

    fn get_changed_input_with_position(&self) -> Option<(InputKind, Vec2)>;

    fn get_input_with_position(&self) -> Option<(InputKind, Vec2)>;
}

type MouseQuery<'w, 's, 'a> = (
    Res<'w, ButtonInput<MouseButton>>,
    Query<'w, 's, &'a Window, With<PrimaryWindow>>,
    Query<'w, 's, (), (With<PrimaryWindow>, Changed<Window>)>,
);

impl PointerQueryExt for MouseQuery<'_, '_, '_> {
    fn get_changed_hover(&self) -> Option<Vec2> {
        let (buttons, window_query, window_changes) = self;

        if (buttons.is_changed() || !window_changes.is_empty())
            && !buttons.pressed(MouseButton::Left)
        {
            if let Ok(window) = window_query.get_single() {
                if let Some(mut position) = window.cursor_position() {
                    position.y = window.height() - position.y;
                    return Some(position);
                }
            }
        }

        None
    }

    fn get_changed_input_with_position(&self) -> Option<(InputKind, Vec2)> {
        let (buttons, _, window_changes) = self;

        if buttons.is_changed() || !window_changes.is_empty() {
            self.get_input_with_position()
        } else {
            None
        }
    }

    fn get_input_with_position(&self) -> Option<(InputKind, Vec2)> {
        let (buttons, window_query, _) = self;

        let input_kind = if buttons.just_pressed(MouseButton::Left) {
            InputKind::Press
        } else if buttons.just_released(MouseButton::Left) {
            InputKind::Release
        } else if buttons.pressed(MouseButton::Left) {
            InputKind::PressedMovement
        } else {
            return None;
        };

        if let Ok(window) = window_query.get_single() {
            if let Some(mut position) = window.cursor_position() {
                position.y = window.height() - position.y;
                return Some((input_kind, position));
            }
        }

        None
    }
}

type TouchQuery<'w, 's, 'a> = (
    Res<'w, Touches>,
    Query<'w, 's, &'a Window, With<PrimaryWindow>>,
    Res<'w, ZoomFactor>,
);

impl PointerQueryExt for TouchQuery<'_, '_, '_> {
    fn get_changed_input_with_position(&self) -> Option<(InputKind, Vec2)> {
        if self.0.is_changed() {
            self.get_input_with_position()
        } else {
            None
        }
    }

    fn get_input_with_position(&self) -> Option<(InputKind, Vec2)> {
        let (touches, window_query, zoom_factor) = self;

        if let Some(mut touch_position) = touches.first_pressed_position() {
            let input_kind = if touches.any_just_pressed() {
                InputKind::Press
            } else {
                InputKind::PressedMovement
            };

            if let Ok(window) = window_query.get_single() {
                touch_position.x *= zoom_factor.x;
                touch_position.y = window.height() - touch_position.y * zoom_factor.y;
                Some((input_kind, touch_position))
            } else {
                None
            }
        } else {
            Some((InputKind::Release, Vec2::default()))
        }
    }
}
