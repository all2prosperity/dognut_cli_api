use pixels::{wgpu::Surface, Frame, Pixels, WinitWindow};
use winit::{event::*, window::WindowBuilder};
use winit::event_loop::{ControlFlow, EventLoop};

mod net;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixels example")
        .with_inner_size(winit::dpi::LogicalSize::new(512.0, 512.0))
        .build(&event_loop)
        .unwrap();

    let surface = Surface::create(&window);
    let mut pixels = Pixels::new(512, 512, surface).expect("Failed to create pixels");
    pixels.get_frame().fill(Rgb8::new(0, 0, 0));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        pixels.render().expect("Failed to render");
    });
}