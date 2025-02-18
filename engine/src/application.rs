use std::collections::HashMap;
use log::{debug, info, warn};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct Application {
    windows: HashMap<WindowId, Window>,
    config: ApplicationConfig,
}

impl Application {
    pub fn new() -> Self{
        let app_config = ApplicationConfig::new("Engine Application".to_string());
        Self {
            windows: HashMap::new(),
            config: app_config,
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.event();
        self.update();
    }

    fn init(&mut self) {
        info!("Initializing application");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap()
    }

    fn event(&mut self) {}

    fn update(&mut self) {}
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            warn!("No windows found in resumed. Creating Window...");
            let window = event_loop.create_window(self.config.window_attributes()).unwrap();
            let window_id = window.id();
            self.windows.insert(window_id, window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                warn!("Window Close requested. Closing Window {:?}", window_id);
                self.windows.remove(&window_id);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            warn!("No active window, exiting...");
            event_loop.exit()
        }
    }
}

pub struct ApplicationConfig {
    pub name: String,
}

impl ApplicationConfig {
    pub fn new(name: String) -> ApplicationConfig {
        ApplicationConfig { name }
    }

    pub fn window_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(self.name.clone())
            .with_inner_size(LogicalSize::new(1280, 720))
    }
}