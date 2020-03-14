extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app.new_window().title("Hello").build().unwrap();
    let gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app

    gl_context.set_window(&window).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    let mut window = Some(window);
    // Run forever
    let mut color = 0.0;

    app.event_loop().run(move |event| match event {
        Event::WindowResized { .. } => gl_context.update_target(), // This call updates the window backbuffer to match the new window size.
        Event::WindowCloseRequested { .. } => {window.take();},
        Event::Draw => {
            unsafe {
                gl.clear_color(1.0, 0.0, color, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            color += 0.01;

            gl_context.swap_buffers(); // Swaps the currently bound window. Blocks if vSync is used
            app.request_frame();
        }
        _ => {}
    });
}
