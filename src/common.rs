use bevy::{input::mouse::MouseButtonInput, prelude::*};

pub struct Solid {}

#[derive(Default)]
pub struct State {
    pub mouse_button_event_reader: EventReader<MouseButtonInput>,
    pub cursor_moved_event_reader: EventReader<CursorMoved>,
}

pub struct MouseLoc(pub Vec2);
