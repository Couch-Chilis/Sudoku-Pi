use crate::ZoomFactor;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    PressedMovement,
    Press,
    Release,
}

pub type PointerQuery<'w, 's, 'a> = (
    Res<'w, ButtonInput<MouseButton>>,
    Res<'w, Touches>,
    Res<'w, ZoomFactor>,
    Query<'w, 's, Ref<'a, Window>, With<PrimaryWindow>>,
);

pub trait PointerQueryExt {
    fn get_changed_hover(&self) -> Option<Vec2>;

    fn get_changed_input_with_position(&self) -> Option<(InputKind, Vec2)>;
}

impl PointerQueryExt for PointerQuery<'_, '_, '_> {
    fn get_changed_hover(&self) -> Option<Vec2> {
        let (buttons, _, zoom_factor, windows) = self;

        let window = windows.single().ok()?;

        if buttons.pressed(MouseButton::Left) || !buttons.is_changed() && !window.is_changed() {
            return None;
        }

        window
            .cursor_position()
            .map(|position| transform_position(position, zoom_factor, &window))
    }

    fn get_changed_input_with_position(&self) -> Option<(InputKind, Vec2)> {
        let (buttons, touches, _, windows) = self;

        if touches.is_changed() {
            println!("Touches changed! {touches:#?}");
            get_touch_input_with_position(self)
        } else if buttons.is_changed() || windows.single().is_ok_and(|window| window.is_changed()) {
            get_button_input_with_position(self)
        } else {
            None
        }
    }
}

fn get_button_input_with_position(query: &PointerQuery) -> Option<(InputKind, Vec2)> {
    let (buttons, _, zoom_factor, windows) = query;

    let input_kind = if buttons.just_pressed(MouseButton::Left) {
        InputKind::Press
    } else if buttons.just_released(MouseButton::Left) {
        InputKind::Release
    } else if buttons.pressed(MouseButton::Left) {
        InputKind::PressedMovement
    } else {
        return None;
    };

    let window = windows.single().ok()?;

    window.cursor_position().map(|position| {
        let position = transform_position(position, zoom_factor, &window);
        (input_kind, position)
    })
}

fn get_touch_input_with_position(query: &PointerQuery) -> Option<(InputKind, Vec2)> {
    let (_, touches, zoom_factor, windows) = query;

    if let Some(touch_position) = touches.first_pressed_position() {
        let input_kind = if touches.any_just_pressed() {
            InputKind::Press
        } else {
            InputKind::PressedMovement
        };

        windows.single().ok().map(|window| {
            let position = transform_position(touch_position, zoom_factor, &window);
            (input_kind, position)
        })
    } else {
        Some((InputKind::Release, Vec2::default()))
    }
}

fn transform_position(position: Vec2, zoom_factor: &ZoomFactor, window: &Window) -> Vec2 {
    Vec2::new(
        position.x * zoom_factor.x,
        window.height() - position.y * zoom_factor.y,
    )
}
