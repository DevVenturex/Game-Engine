use std::env;
use engine::Application;

fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();

    let mut app = Application::new();
    pollster::block_on(app.run());
}
