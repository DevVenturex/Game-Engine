use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window as WinitWindow, WindowAttributes};
use crate::stack::Stack;

pub struct Window {
    title: String,
    size: PhysicalSize<u32>,
    resizable: bool,
    decorations: bool,
    winit_window: WinitWindow,
}

impl Window {
    pub fn new(event_loop: &ActiveEventLoop, settings: WindowSettings) -> Self {
        let winit_window = event_loop.create_window(settings.attributes()).unwrap();
        Self {
            title: winit_window.title(),
            size: winit_window.outer_size(),
            resizable: winit_window.is_resizable(),
            decorations: winit_window.is_decorated(),
            winit_window,
        }
    }

    pub fn event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => ()
        }
    }

    pub fn update(&mut self) {

    }

    pub fn cleanup(&mut self) {

    }
}

pub struct WindowSettings {
    title: String,
    size: PhysicalSize<u32>,
    resizable: bool,
    decorations: bool,
}

impl WindowSettings {
    pub fn new(title: String, size: PhysicalSize<u32>, resizable: bool, decorations: bool) -> Self {
        Self {
            title, size, resizable, decorations,
        }
    }

    pub fn attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(self.title.clone())
            .with_inner_size(PhysicalSize::new(self.size.width, self.size.height))
            .with_resizable(self.resizable)
            .with_decorations(self.decorations)
    }
}