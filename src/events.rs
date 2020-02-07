use crate::keys::Key;

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub enum Event {
    Draw,
    KeyDown {
        key: Key,
        scancode: u32,
    },
    KeyUp {
        key: Key,
        scancode: u32,
    },
    MinimizedWindow,
    MaximizedWindow,
    ResizedWindow {
        width: u32,
        height: u32,
    },
    MouseMoved {
        x: f32,
        y: f32,
    },
    MouseDown {
        button: MouseButton,
    },
    MouseUp {
        button: MouseButton,
    },
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added
}