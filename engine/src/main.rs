use std::env;
use winit::event_loop::EventLoop;
use crate::application::Application;

mod application;

fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();
    let mut app = Application::new();
    app.run();
}
