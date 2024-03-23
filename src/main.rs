// Include 'winit' window manager components to allow for window creation
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

// Main function
fn main() {
    // Event loop object provides a way to retrieve events from the system/any registered windows
    let event_loop = EventLoop::new().unwrap();

    // Window object represents a window
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Control flow indicates the desired behaviour of the event loop when Event::AboutToWait is emitted i.e. when the event loop is about to block and wait for new events because its done processing events
    event_loop.set_control_flow(ControlFlow::Wait);

    // Executes the 'run()' method of the event loop object to execute the event loop, running it in the calling thread and calling the event handler closure to dispatch any pending events
    // 'event' is the event itself, 'elwt' is the event loop window target that associated windows with the particular event loop
    event_loop.run(move |event, elwt| {
        match event {
            // Event::WindowEvent describes an event from a window
            Event::WindowEvent {
                // WindowEvent::CloseRequested describes a window event where the window requests to close
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // If a window close is requested, event loop window target is used to target the window and close it using 'exit()' method
                println!("Window close requested.");
                elwt.exit();
            },

            // Event::AboutToWait describes when the event loop is about to block and wait for new events since all events have already been processed
            Event::AboutToWait => {
                window.request_redraw();
            },

            // Event::RedrawRequested is an event emitted when a window should be redrawn i.e. where the OS has performed operation that invalidated the window's contents or the application as explicitly requested a redraw via Window::request_redraw
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {

            },

            _ => {}
        }
    });
}
