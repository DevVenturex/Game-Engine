

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    pub fn empty(&self) -> Stack<T> {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.data.insert(index, value);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        Some(self.data.remove(index))
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}