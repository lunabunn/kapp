extern crate gl;
extern crate windowing;
use windowing::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new();
    let window = window_manager.new_window("Window Title").unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}