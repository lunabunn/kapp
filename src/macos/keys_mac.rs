use crate::buttons::{Button, Button::*};

pub fn virtual_keycode_to_key(key_in: u16) -> Button {
    match key_in {
        0x1D => Digit0,
        0x12 => Digit1,
        0x13 => Digit2,
        0x14 => Digit3,
        0x15 => Digit4,
        0x17 => Digit5,
        0x16 => Digit6,
        0x1A => Digit7,
        0x1C => Digit8,
        0x19 => Digit9,
        0x00 => A,
        0x0B => B,
        0x08 => C,
        0x02 => D,
        0x0E => E,
        0x03 => F,
        0x05 => G,
        0x04 => H,
        0x22 => I,
        0x26 => J,
        0x28 => K,
        0x25 => L,
        0x2E => M,
        0x2D => N,
        0x1F => O,
        0x23 => P,
        0x0C => Q,
        0x0F => R,
        0x01 => S,
        0x11 => T,
        0x20 => U,
        0x09 => V,
        0x0D => W,
        0x07 => X,
        0x10 => Y,
        0x06 => Z,
        0x27 => Backquote,
        0x2A => Backslash,
        0x2B => Comma,
        0x18 => Equal,
        0x32 => Unknown, // What is this? Sokol calls it 'grave'
        0x21 => BracketLeft,
        0x1B => Minus,
        0x2F => Period,
        0x1E => BracketRight,
        0x29 => Semicolon,
        0x2C => Slash,
        0x0A => Unknown, // What is this?
        0x33 => Backspace,
        0x39 => CapsLock,
        0x75 => Delete,
        0x7D => Down,
        0x77 => End,
        0x24 => Return, // Should this be called enter? 
        0x35 => Escape,
        0x7A => F1,
        0x78 => F2,
        0x63 => F3,
        0x76 => F4,
        0x60 => F5,
        0x61 => F6,
        0x62 => F7,
        0x64 => F8,
        0x65 => F9,
        0x6D => F10,
        0x67 => F11,
        0x6F => F12,
        0x69 => F13,
        0x6B => F14,
        0x71 => F15,
        0x6A => F16,
        0x40 => F17,
        0x4F => F18,
        0x50 => F19,
        0x5A => F20,
        0x73 => Home,
        0x72 => Insert,
        0x7B => Left,
        0x3A => Unknown, // Left Alt
        0x3B => LeftControl,
        0x38 => LeftShift,
        0x37 => Meta, // Left command key
        0x6E => Menu,
        0x47 => NumLock,
        0x79 => PageDown,
        0x74 => PageUp,
        0x7C => Right,
        0x3D => Unknown, // Right alt
        0x3E => RightControl,
        0x3C => RightShift,
        0x36 => Meta, // Right command key
        0x31 => Space,
        0x30 => Tab,
        0x7E => Up,
        0x52 => NumPad0,
        0x53 => NumPad1,
        0x54 => NumPad2,
        0x55 => NumPad3,
        0x56 => NumPad4,
        0x57 => NumPad5,
        0x58 => NumPad6,
        0x59 => NumPad7,
        0x5B => NumPad8,
        0x5C => NumPad9,
        0x45 => NumPadAdd,
        0x41 => NumPadDecimal,
        0x4B => NumPadDivide,
        0x4C => NumPadEnter,
        0x51 => NumPadEquals,
        0x43 => NumPadMultiply,
        0x4E => NumPadSubtract,
        _ => Unknown
    }
}