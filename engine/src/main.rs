use std::env;
use crate::application::Application;

mod application;
mod layers;
pub mod egui;

fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();
    let mut app = Application::new();
    app.run();
}
