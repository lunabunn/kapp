/// These are the core functions to be implemented by each platform.
use crate::{raw_window_handle::RawWindowHandle, Cursor, WindowId, WindowParameters};
pub trait PlatformApplicationTrait {
    type EventLoop: PlatformEventLoopTrait;

    fn new() -> Self;
    fn event_loop(&mut self) -> Self::EventLoop;

    /// Sets window position in physical coordinates on its current screen.
    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32);
    /// Sets window size with physical coordinates.
    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32);
    fn set_window_title(&mut self, window_id: WindowId, title: &str);
    fn minimize_window(&mut self, window_id: WindowId);
    fn maximize_window(&mut self, window_id: WindowId);
    fn fullscreen_window(&mut self, window_id: WindowId);
    /// Returns the window to the state where it's not minimized, maximized, or fullscreen
    fn restore_window(&mut self, window_id: WindowId);
    fn close_window(&mut self, window_id: WindowId);

    fn get_window_size(&mut self, _window_id: WindowId) -> (u32, u32);
    fn get_window_scale(&mut self, _window_id: WindowId) -> f64;

    /// Requests that the a Draw event be sent for the window.
    /// Draw events should either be sent at the end of an event loop,
    /// or in response to a system redraw request.
    /// If multiple window redraws are requested no ordering should be assumed.
    fn redraw_window(&mut self, window_id: WindowId);

    /// Sets the mouse position in physical coordinates in relation to the screen.
    fn set_mouse_position(&mut self, x: u32, y: u32);
    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId;

    /// Request that the application should quit immediately.
    /// This should be possible to be called multiple times without error.
    /// The actual termination initiation should be postponed until the end of the event loop.
    /// If termination is initiated while the program closure is active then
    /// things may be borrowed multiple times.
    /// The termination should occur before any requested draw events.
    fn quit(&self);

    /// Sets the cursor in a way that persists between all windows for the current program.
    fn set_cursor(&mut self, cursor: Cursor);

    /// Hides the cursor or this application until a call to show cursor.
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);

    /// Returns a RawWindowHandle as defined in the raw_window_handle crate
    /// https://github.com/rust-windowing/raw-window-handle
    fn raw_window_handle(&self, window: WindowId) -> RawWindowHandle;
}

pub trait PlatformEventLoopTrait {
    /// Runs until the application quits.
    fn run(&self, callback: Box<dyn FnMut(crate::Event)>);
}