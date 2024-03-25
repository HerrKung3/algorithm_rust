//双端队列
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>
}

//Vec [尾部...首部]
impl<T> Deque<T> {
     pub fn new(cap: usize) -> Self {
        Deque { cap, data: Vec::with_capacity(cap) }
    }

    //Vec首部作为队首
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    //Vec尾部作为队首
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) == 0 {
            return None;
        }else {
            return  Some(self.data.remove(0));
        } 
    }

    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}