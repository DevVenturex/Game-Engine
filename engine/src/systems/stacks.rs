use std::ops::Deref;
use crate::stack::Stack;
use crate::systems::layers::Layer;
use crate::windows::Window;

pub type WindowStack = Stack<Window>;
type StackedLayer = Stack<Box<dyn Layer>>;

pub struct LayerStack {
    stack: StackedLayer,
    overlay_start: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            stack: StackedLayer::new(),
            overlay_start: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.stack.insert(self.overlay_start, layer);
        self.overlay_start += 1;
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.stack.push(overlay);
    }

    pub fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        let layer = self.stack.remove(self.overlay_start - 1).unwrap();
        self.overlay_start -= 1;
        Some(layer)
    }

    pub fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        self.stack.pop()
    }

    pub fn overlay_start(&self) -> usize {
        self.overlay_start
    }

    pub fn stack(&self) -> &StackedLayer {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut StackedLayer {
        &mut self.stack
    }
}