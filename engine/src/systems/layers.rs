use log::info;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

pub trait Layer {
    fn init(&mut self);
    fn update(&mut self);
    fn event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent);
    fn cleanup(&mut self);
}

pub struct RenderLayer {

}

impl RenderLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layer for RenderLayer {
    fn init(&mut self) {

    }

    fn update(&mut self) {

    }

    fn event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {

    }

    fn cleanup(&mut self) {

    }
}