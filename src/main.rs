mod window;
mod render;
mod config;
mod errors;

fn main() {
    // let (event_loop, window) = window::init::initialize_winit_context();
    // window::event_loop::run(event_loop, window);
    config::parser::parse_toml().unwrap();
    render::renderer::render();
}
