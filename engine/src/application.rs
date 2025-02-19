use std::collections::HashMap;
use std::process::ExitStatus;
use std::sync::Arc;
use egui::WidgetInfo;
use egui_wgpu::{wgpu, ScreenDescriptor};
use egui_wgpu::wgpu::SurfaceError;
use log::{debug, info, warn};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::egui::renderer::EguiRenderer;
use crate::layers::layer_stack::{Layer, LayerStack};

pub enum UserEvent {
    PushLayer(Box<dyn Layer + 'static>), PushOverlay(Box<dyn Layer + 'static>),
}

pub struct Application {
    windows: HashMap<WindowId, Arc<Window>>,
    layer_stack: LayerStack,
    event_loop_proxy: Option<EventLoopProxy<UserEvent>>,
}

impl Application {
    pub fn new() -> Self{
        Self {
            windows: HashMap::new(),
            layer_stack: LayerStack::new(),
            event_loop_proxy: None,
        }
    }

    async fn set_window(&mut self, window: Window) {
        let window_id = window.id();
        let window = Arc::new(window);
        let initial_width = 1280;
        let initial_height = 720;

        let _ = window.request_inner_size(PhysicalSize::new(initial_width, initial_height));

        self.windows.insert(window_id, window);
    }

    pub fn run(&mut self) {
        self.init();
        self.update();
    }

    fn init(&mut self) {
        info!("Initializing application");
        let event_loop: EventLoop<UserEvent> = EventLoop::<UserEvent>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        self.event_loop_proxy = Some(event_loop.create_proxy());

        event_loop.run_app(self).unwrap();
    }

    fn update(&mut self) {
        for (_, window) in &mut self.windows.iter_mut() {
            window.request_redraw();
        }
    }

    fn push_layer(&mut self, layer: Box<dyn Layer>) {
        debug!("Pushed layer");
        self.layer_stack.push_layer(layer);
    }

    fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        debug!("Pushed overlay");
        self.layer_stack.push_overlay(overlay);
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            warn!("No windows found in resumed. Creating Window...");
            let window = event_loop.create_window(ApplicationConfig::default().window_attributes()).unwrap();
            pollster::block_on(self.set_window(window));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::PushLayer(layer) => self.push_layer(layer),
            UserEvent::PushOverlay(overlay) => self.push_overlay(overlay),
        }

    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

        match event {
            WindowEvent::CloseRequested => {
                warn!("Window Close requested. Closing Window {:?}", window_id);
                self.windows.remove(&window_id);
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            warn!("No active window, exiting...");
            event_loop.exit()
        }
        self.update();
    }
}

pub struct ApplicationState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub scale_factor: f32,
}

impl ApplicationState {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
        _window: &Window,
        size: PhysicalSize<u32>,
    ) -> Self {
        let power_ref = wgpu::PowerPreference::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_ref,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let features = wgpu::Features::empty();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: features,
                    required_limits: Default::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let selected_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let swapchain_format = swapchain_capabilities
            .formats
            .iter()
            .find(|d| **d == selected_format)
            .expect("Failed to select proper surface texture format");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 0,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&device, &surface_config);

        let scale_factor = 1.0;

        Self {
            device,
            queue,
            surface,
            surface_config,
            scale_factor,
        }
    }

    pub fn resize_surface(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}

#[derive(Default)]
pub struct ApplicationConfig {}

impl ApplicationConfig {
    pub fn new() -> ApplicationConfig {
        Self {}
    }

    pub fn window_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title("Engine Application")
            .with_inner_size(LogicalSize::new(1280, 720))
    }
}