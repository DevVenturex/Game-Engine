use std::sync::Arc;
use egui_wgpu::{wgpu, ScreenDescriptor};
use egui_wgpu::wgpu::{CommandEncoder, Device, TextureFormat, TextureView};
use log::info;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::application::ApplicationState;
use crate::egui::renderer::EguiRenderer;
use crate::layers::layer_stack::Layer;

pub mod renderer;

pub struct EguiLayer {
    egui_renderer: EguiRenderer,
    app_state: ApplicationState,
    window: Arc<&'static Window>,
    encoder: CommandEncoder,
    surface_view: TextureView,
    screen_descriptor: &'static ScreenDescriptor
}

impl EguiLayer {
    pub fn new(
        app_state: ApplicationState,
        device: &Device,
        surface_config: wgpu::SurfaceConfiguration,
        window: Arc<&'static Window>,
        encoder: CommandEncoder,
        surface_view: TextureView,
        screen_descriptor: &'static ScreenDescriptor
    ) -> Self {
        let egui_renderer = EguiRenderer::new(&device, surface_config.format, None, 1, window.as_ref());

        Self {
            egui_renderer,
            app_state,
            window,
            encoder,
            surface_view,
            screen_descriptor,
        }
    }
}

impl Layer for EguiLayer {
    fn init(&mut self) {
    }

    fn update(&mut self) {
        let window = *self.window.as_ref();
        self.egui_renderer.begin_frame(window);
        egui::Window::new("winit + egui + wgpu says hello!")
            .resizable(true)
            .vscroll(true)
            .default_open(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.label("Label!");

                if ui.button("Button!").clicked() {
                    info!("Clicked button!");
                }

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Pixels per point: {}",
                        self.egui_renderer.context().pixels_per_point()
                    ));
                    if ui.button("-").clicked() {
                        self.app_state.scale_factor = (self.app_state.scale_factor - 0.1).max(0.3);
                    }
                    if ui.button("+").clicked() {
                        self.app_state.scale_factor = (self.app_state.scale_factor + 0.1).min(3.0);
                    }
                });
            });
        self.egui_renderer.end_frame_and_draw(
            &self.app_state.device,
            &self.app_state.queue,
            &mut self.encoder,
            window,
            &self.surface_view,
            &self.screen_descriptor
        );
    }

    fn event(&mut self, event: &WindowEvent) {
        self.egui_renderer.event(self.window.as_ref(), event)
    }
}