use crate::platform::*;
use crate::platform::{PlatformApplicationTrait, PlatformEventLoopTrait};
use crate::state_tracker::StateTracker;
use std::cell::RefCell;
use std::rc::Rc;

/// A handle used to do things like quit,
/// request a new frame, or create windows.
#[derive(Clone)]
pub struct Application {
    pub(crate) platform_application: Rc<RefCell<PlatformApplication>>,
    state_tracker: Rc<RefCell<StateTracker>>,
}

/// Create an Application and EventLoop.
pub fn initialize() -> (Application, EventLoop) {
    let platform_application = Rc::new(RefCell::new(PlatformApplication::new()));
    let platform_event_loop = platform_application.borrow_mut().event_loop();
    let state_tracker = Rc::new(RefCell::new(StateTracker::new()));
    (
        Application {
            platform_application: platform_application.clone(),
            state_tracker: state_tracker.clone(),
        },
        EventLoop {
            platform_event_loop,
            state_tracker: state_tracker.clone(),
        },
    )
}

impl Application {
    /// Returns a new window builder.
    /// Call .build() on the window builder to complete the creation of the window.
    /// See [`crate::window_builder::WindowBuilder`] for more ways to setup a window.
    pub fn new_window(&self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self)
    }

    /// Immediately quits the application.
    pub fn quit(&self) {
        self.platform_application.borrow().quit();
    }

    /// Sets the mouse position relative to the screen.
    /// Coordinates are expressed in physical coordinates.
    pub fn set_mouse_position(&self, x: u32, y: u32) {
        self.platform_application
            .borrow_mut()
            .set_mouse_position(x, y);
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        self.platform_application.borrow_mut().set_cursor(cursor);
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        if visible {
            self.platform_application.borrow_mut().show_cursor();
        } else {
            self.platform_application.borrow_mut().hide_cursor();
        }
    }

    /// Returns if the key is currently pressed
    pub fn key(&self, key: Key) -> bool {
        self.state_tracker.borrow().key(key)
    }

    /// Returns true if the key has been pressed since the last draw
    pub fn key_down(&self, key: Key) -> bool {
        self.state_tracker.borrow().key_down(key)
    }

    /// Returns true if all the keys specified been pressed since the last draw.
    /// Right now this doesn't work perfectly for keyboard shortcuts because
    /// the different modifier keys are split out into their left and right versions.
    pub fn keys_down(&self, keys: &[Key]) -> bool {
        self.state_tracker.borrow().keys_down(keys)
    }

    /// Returns true if the mouse button is pressed
    pub fn mouse_button(&self, button: MouseButton) -> bool {
        self.state_tracker.borrow().mouse_button(button)
    }

    /// Returns true if the mouse button has been pressed since the last draw
    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.state_tracker.borrow().mouse_button_down(button)
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        self.state_tracker.borrow().mouse_position()
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}

/// Call the 'run' or 'run_async' function on an EventLoop instance to start your program.
pub struct EventLoop {
    platform_event_loop: PlatformEventLoop,
    state_tracker: Rc<RefCell<StateTracker>>,
}

impl EventLoop {
    /// Run the application. The callback is called for each new event.
    pub fn run<T>(&self, mut callback: T)
    where
        T: 'static + FnMut(Event),
    {
        let state_tracker = self.state_tracker.clone();
        let callback_wrapper = move |event| {
            state_tracker.borrow_mut().handle_event(event);
            callback(event);
            state_tracker.borrow_mut().post_program_callback(event);
        };
        self.platform_event_loop.run(Box::new(callback_wrapper));
    }
}
