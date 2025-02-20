use std::collections::HashMap;
use std::os::unix::process;
use std::process::ExitStatus;
use std::rc::Rc;
use std::sync::Arc;
use egui::WidgetInfo;
use egui_wgpu::ScreenDescriptor;
use log::{debug, error, info, warn};
use wgpu::CommandEncoderDescriptor;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::layers::layer_stack::{Layer, LayerStack};

pub enum UserEvent {
    PushLayer(Box<dyn Layer + 'static>), PushOverlay(Box<dyn Layer + 'static>),
}

pub struct Application {
    window_states: HashMap<WindowId, State>,
    layer_stack: LayerStack,
    event_loop_proxy: Option<EventLoopProxy<UserEvent>>,
}

impl Application {
    pub fn new() -> Self{
        Self {
            window_states: HashMap::new(),
            layer_stack: LayerStack::new(),
            event_loop_proxy: None,
        }
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
        for (_, window) in &mut self.window_states.iter_mut() {
            window.window.request_redraw();
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
        if self.window_states.is_empty() {
            let window = Arc::new(event_loop.create_window(
                WindowAttributes::default()
                    .with_title("App Window")
                    .with_inner_size(PhysicalSize::new(1280, 720)),
            ).unwrap());
            let window_id = window.id();
            let state = pollster::block_on(State::new(window.clone()));
            self.window_states.insert(window_id, state);
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
                self.window_states.remove(&window_id);
            },
            WindowEvent::Resized(new_size) => {
                self.window_states.get_mut(&window_id).unwrap().resize(new_size);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_states.is_empty() {
            warn!("No active window, exiting...");
            event_loop.exit()
        }
        self.update();
        self.window_states
            .iter_mut()
            .for_each(|(_, state)| {
                match state.render() {
                    Ok(_) => {},
                    Err(
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                    ) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                        error!("OutOfMemory");
                        event_loop.exit()
                    }
                    Err(wgpu::SurfaceError::Timeout) => {
                        warn!("Surface timeout")
                    }
                }
            })
    }
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}