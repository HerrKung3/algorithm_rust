#[derive(Debug)]
pub struct Quenue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Quenue<T> {
    pub fn new(cap: usize) -> Self {
        Quenue { cap, data: Vec::with_capacity(cap) }
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        }else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

}