use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crate::layers::{Layer, RenderLayer};
use crate::stacks::{LayerStack, WindowStack};
use crate::systems::windows::{Window, WindowSettings};

pub struct Application {
    window_stack: WindowStack,
    layer_stack: LayerStack,
}

impl Application {
    pub fn new() -> Application {
        Application {
            window_stack: WindowStack::new(),
            layer_stack: LayerStack::new(),
        }
    }

    pub async fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap()
    }

    pub fn init(&mut self) {
        self.layer_stack.push_layer(Box::new(RenderLayer::new()));

        self.layer_stack.stack_mut().data_mut()
            .iter_mut()
            .for_each(|layer| layer.init())
    }

    pub fn update(&mut self) {
        self.window_stack.data_mut().iter_mut().for_each(|window| window.update());

        self.layer_stack.stack_mut().data_mut()
            .iter_mut()
            .for_each(|layer| layer.update())
    }

    pub fn cleanup(&mut self) {
        self.window_stack.data_mut().iter_mut().for_each(|window| window.cleanup());

        self.layer_stack.stack_mut().data_mut()
            .iter_mut()
            .for_each(|layer| layer.cleanup())
    }

    pub fn exit(&mut self) {}

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let settings = WindowSettings::new("Engine Window".to_string(), PhysicalSize::new(1280, 720), true, true);
        let window = Window::new(event_loop, settings);
        self.window_stack.push(window);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
        self.init();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        self.layer_stack.stack_mut().data_mut()
            .iter_mut()
            .for_each(|layer| layer.event(event_loop, event.clone()));
        self.window_stack.data_mut().iter_mut().for_each(|window| window.event(event_loop, event.clone()))
    }
}