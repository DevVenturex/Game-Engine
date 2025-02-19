use winit::event::WindowEvent;

pub trait Layer {
    fn init(&mut self);
    fn update(&mut self);
    fn event(&mut self, event: &WindowEvent);
}

pub struct LayerStack {
    pub stack: Vec<Box<dyn Layer>>,
    overlay_start: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        LayerStack { stack: Vec::new(), overlay_start: 0 }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.stack.insert(self.overlay_start, layer);
        self.overlay_start += 1;
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.stack.push(overlay);
    }

    pub fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        if self.overlay_start > 0 {
            self.overlay_start -= 1;
            Some(self.stack.remove(self.overlay_start))
        } else {
            None
        }
    }

    pub fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        if self.stack.len() > self.overlay_start {
            Some(self.stack.pop().unwrap())
        } else {
            None
        }
    }
}