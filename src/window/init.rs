use winit::{event_loop::EventLoop, window::Window, window::WindowBuilder};

pub fn initialize_winit_context() -> (EventLoop<()>, Window) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    (event_loop, window)
}
