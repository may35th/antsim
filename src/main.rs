use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Create a window
    let window = WindowBuilder::new()
        .with_title("Rendering Window")
        .build(&event_loop)
        .unwrap();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}