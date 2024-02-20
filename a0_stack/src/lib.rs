#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { size: 0, data: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0{
            return None;
        }
        self.data.get(self.size-1)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}